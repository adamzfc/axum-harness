//! libsql local database Port trait.
//!
//! Used by native-tauri (Tauri app) for local embedded storage.
//! Uses standard SQLite-compatible SQL.
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::pin::Pin;

/// Transaction guard for atomic SQL operations.
///
/// Provides `execute` on the same underlying connection that issued
/// `BEGIN`. Call `commit` to persist or `rollback` to discard.
/// Dropping without calling either defaults to rollback.
///
/// Note: `query` is intentionally excluded to maintain dyn-compatibility
/// (generic methods break `dyn SqlTransaction`). Use the parent port's
/// `query` for post-commit reads — transactions only need atomic writes.
#[async_trait]
pub trait SqlTransaction: Send + Sync {
    /// Execute a SQL statement inside the transaction.
    async fn execute(&self, sql: &str, params: Vec<String>) -> Result<u64, LibSqlError>;

    /// Commit the transaction (persist all changes).
    async fn commit(self: Box<Self>) -> Result<(), LibSqlError>;

    /// Roll back the transaction (discard all changes).
    async fn rollback(self: Box<Self>) -> Result<(), LibSqlError>;
}
/// Error type for libsql operations.
pub type LibSqlError = Box<dyn std::error::Error + Send + Sync>;
/// libsql port — abstracts local embedded database access.
///
/// Implementations live in data-adapters crates.
/// Uses standard SQLite SQL (unlike SurrealDB's SurrealQL).
#[async_trait]
pub trait LibSqlPort: Send + Sync {
    /// Verify the database connection is alive.
    async fn health_check(&self) -> Result<(), LibSqlError>;
    /// Execute a SQL statement (INSERT, UPDATE, DELETE, DDL).
    /// Returns the number of affected rows.
    async fn execute(&self, sql: &str, params: Vec<String>) -> Result<u64, LibSqlError>;
    /// Execute multiple SQL statements as a single batch on the same connection.
    ///
    /// Supports `BEGIN...COMMIT` for transactional multi-statement execution.
    /// Used by repository adapters for atomic CAS + outbox writes where
    /// `RETURNING` results are bridged via temp tables.
    ///
    /// Connection isolation: implementations must ensure all statements in the
    /// batch execute on the same underlying connection. EmbeddedTurso achieves
    /// this via mutex-based locking; remote Turso via connection pooling.
    ///
    /// NOTE: A typed transaction API (`begin()`/`commit()` on `Connection`)
    /// would provide stronger compile-time guarantees and is deferred for now
    /// (requires connection-level ownership model, not just batch-level).
    async fn execute_batch(&self, sql: &str) -> Result<(), LibSqlError>;
    /// Begin a typed transaction on a dedicated connection.
    ///
    /// All `execute`/`query` calls on the returned `SqlTransaction` run
    /// on the **same** underlying connection. Call `commit` to persist or
    /// `rollback` to discard. Dropping without either defaults to rollback.
    ///
    /// This provides stronger compile-time guarantees than `execute_batch`
    /// (which accepts raw `BEGIN;...;COMMIT;` SQL strings).
    async fn begin(&self) -> Result<Box<dyn SqlTransaction>, LibSqlError>;
    /// Execute a SQL query (SELECT) returning deserialized rows.
    async fn query<T: DeserializeOwned + Send + Sync>(
        &self,
        sql: &str,
        params: Vec<String>,
    ) -> Result<Vec<T>, LibSqlError>;
}
