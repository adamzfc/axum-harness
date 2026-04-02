//! Counter service — LibSQL-backed implementation.

use async_trait::async_trait;
use domain::ports::lib_sql::LibSqlPort;
use feature_counter::{CounterError, CounterService};

/// Counter table migration SQL.
pub const COUNTER_MIGRATION: &str = "CREATE TABLE IF NOT EXISTS counter (id INTEGER PRIMARY KEY, value INTEGER NOT NULL DEFAULT 0, updated_at TEXT NOT NULL DEFAULT (datetime('now')))";

/// CounterService backed by LibSqlPort.
pub struct LibSqlCounterService<P: LibSqlPort> {
    port: P,
}

impl<P: LibSqlPort> LibSqlCounterService<P> {
    pub fn new(port: P) -> Self {
        Self { port }
    }
}

#[async_trait]
impl<P: LibSqlPort> CounterService for LibSqlCounterService<P> {
    async fn get_value(&self) -> Result<i64, CounterError> {
        let rows: Vec<(i64,)> = self
            .port
            .query("SELECT value FROM counter WHERE id = 1", vec![])
            .await
            .map_err(CounterError::Database)?;
        Ok(rows.first().map(|r| r.0).unwrap_or(0))
    }

    async fn increment(&self) -> Result<i64, CounterError> {
        self.port
            .execute(
                "INSERT INTO counter (id, value, updated_at) VALUES (1, 1, datetime('now')) ON CONFLICT(id) DO UPDATE SET value = value + 1, updated_at = datetime('now')",
                vec![],
            )
            .await
            .map_err(CounterError::Database)?;
        self.get_value().await
    }

    async fn decrement(&self) -> Result<i64, CounterError> {
        self.port
            .execute(
                "UPDATE counter SET value = value - 1, updated_at = datetime('now') WHERE id = 1",
                vec![],
            )
            .await
            .map_err(CounterError::Database)?;
        self.get_value().await
    }

    async fn reset(&self) -> Result<i64, CounterError> {
        self.port
            .execute(
                "UPDATE counter SET value = 0, updated_at = datetime('now') WHERE id = 1",
                vec![],
            )
            .await
            .map_err(CounterError::Database)?;
        Ok(0)
    }
}
