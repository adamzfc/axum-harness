//! CounterRepository implementation backed by libsql (Turso embedded).
//!
//! This adapter translates the abstract CounterRepository trait into
//! concrete SQL operations. It handles:
//! - Counter upsert on first access (INSERT ... ON CONFLICT)
//! - Atomic increment/decrement via SQL UPDATE
//! - Timestamp management via datetime('now')

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::ports::lib_sql::LibSqlPort;
use serde::Deserialize;

use crate::domain::{Counter, CounterId};
use crate::ports::{CounterRepository, RepositoryError};

/// Raw row shape from the counter table.
#[derive(Debug, Deserialize)]
struct CounterRow {
    tenant_id: String,
    value: i64,
    updated_at: String,
}

/// Minimal row shape for value-only queries.
#[derive(Debug, Deserialize)]
struct ValueRow {
    value: i64,
}

/// CounterRepository backed by a libsql port.
///
/// This is the **primary** repository implementation used in Phase 0
/// where the monolith uses embedded Turso (libsql) for storage.
pub struct LibSqlCounterRepository<P: LibSqlPort> {
    port: P,
}

impl<P: LibSqlPort> LibSqlCounterRepository<P> {
    pub fn new(port: P) -> Self {
        Self { port }
    }

    /// Run the counter table migration (idempotent).
    ///
    /// This should be called at application startup by the composition root.
    pub async fn migrate(&self) -> Result<(), RepositoryError> {
        const COUNTER_MIGRATION: &str =
            "CREATE TABLE IF NOT EXISTS counter (\
                tenant_id TEXT PRIMARY KEY,\
                value INTEGER NOT NULL DEFAULT 0,\
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))\
            )";
        self.port.execute(COUNTER_MIGRATION, vec![]).await?;
        Ok(())
    }
}

#[async_trait]
impl<P: LibSqlPort> CounterRepository for LibSqlCounterRepository<P> {
    async fn load(&self, id: &CounterId) -> Result<Option<Counter>, RepositoryError> {
        let rows: Vec<CounterRow> = self
            .port
            .query(
                "SELECT tenant_id, value, updated_at FROM counter WHERE tenant_id = ?",
                vec![id.as_str().to_string()],
            )
            .await?;

        let row = match rows.first() {
            Some(r) => r,
            None => return Ok(None),
        };

        let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)
            .or_else(|_| DateTime::parse_from_str(&row.updated_at, "%Y-%m-%d %H:%M:%S"))
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(Some(Counter {
            id: CounterId::new(&row.tenant_id),
            value: row.value,
            updated_at,
        }))
    }

    async fn increment(
        &self,
        id: &CounterId,
        _now: DateTime<Utc>,
    ) -> Result<i64, RepositoryError> {
        // Upsert: create row if missing, increment if exists
        self.port
            .execute(
                "INSERT INTO counter (tenant_id, value, updated_at) \
                 VALUES (?, 1, datetime('now')) \
                 ON CONFLICT(tenant_id) DO UPDATE SET \
                     value = value + 1, \
                     updated_at = datetime('now')",
                vec![id.as_str().to_string()],
            )
            .await?;

        // Read back the new value
        let rows: Vec<ValueRow> = self
            .port
            .query(
                "SELECT value FROM counter WHERE tenant_id = ?",
                vec![id.as_str().to_string()],
            )
            .await?;

        Ok(rows.first().map(|r| r.value).unwrap_or(0))
    }

    async fn decrement(
        &self,
        id: &CounterId,
        _now: DateTime<Utc>,
    ) -> Result<i64, RepositoryError> {
        self.port
            .execute(
                "UPDATE counter SET value = value - 1, updated_at = datetime('now') \
                 WHERE tenant_id = ?",
                vec![id.as_str().to_string()],
            )
            .await?;

        let rows: Vec<ValueRow> = self
            .port
            .query(
                "SELECT value FROM counter WHERE tenant_id = ?",
                vec![id.as_str().to_string()],
            )
            .await?;

        Ok(rows.first().map(|r| r.value).unwrap_or(0))
    }

    async fn reset(&self, id: &CounterId, _now: DateTime<Utc>) -> Result<(), RepositoryError> {
        self.port
            .execute(
                "UPDATE counter SET value = 0, updated_at = datetime('now') \
                 WHERE tenant_id = ?",
                vec![id.as_str().to_string()],
            )
            .await?;
        Ok(())
    }

    async fn upsert(&self, counter: &Counter) -> Result<(), RepositoryError> {
        self.port
            .execute(
                "INSERT INTO counter (tenant_id, value, updated_at) \
                 VALUES (?, ?, ?) \
                 ON CONFLICT(tenant_id) DO UPDATE SET \
                     value = excluded.value, \
                     updated_at = excluded.updated_at",
                vec![
                    counter.id.as_str().to_string(),
                    counter.value.to_string(),
                    counter.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                ],
            )
            .await?;
        Ok(())
    }
}
