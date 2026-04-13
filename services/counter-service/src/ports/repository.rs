//! CounterRepository — abstract persistence for the Counter aggregate.
//!
//! This trait is the **only** way the counter application layer talks to storage.
//! Implementations may use libsql, Turso cloud, SurrealDB, or in-memory stubs.

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::super::domain::{Counter, CounterId};

/// Errors that can occur during repository operations.
///
/// This is intentionally generic — the repository implementation maps its
/// specific errors (SQL, HTTP, etc.) into this type.
pub type RepositoryError = Box<dyn std::error::Error + Send + Sync>;

/// Abstract repository for Counter persistence.
///
/// ## Responsibilities
/// - Store and retrieve Counter aggregates by CounterId
/// - Execute atomic increment/decrement/reset operations
/// - Never leak business logic (no "default tenant" concepts here)
#[async_trait]
pub trait CounterRepository: Send + Sync {
    /// Load a counter by its ID. Returns None if not found.
    async fn load(&self, id: &CounterId) -> Result<Option<Counter>, RepositoryError>;

    /// Atomically increment a counter. Creates it if it doesn't exist.
    /// Returns the new value after increment.
    async fn increment(&self, id: &CounterId, now: DateTime<Utc>) -> Result<i64, RepositoryError>;

    /// Atomically decrement a counter.
    async fn decrement(&self, id: &CounterId, now: DateTime<Utc>) -> Result<i64, RepositoryError>;

    /// Reset a counter to zero.
    async fn reset(&self, id: &CounterId, now: DateTime<Utc>) -> Result<(), RepositoryError>;

    /// Upsert a counter (used by application layer on first access).
    async fn upsert(&self, counter: &Counter) -> Result<(), RepositoryError>;
}
