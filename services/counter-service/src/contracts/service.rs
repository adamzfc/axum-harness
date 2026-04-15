//! Counter service trait — moved from packages/features/counter
//!
//! ## Boundary Mapping
//! - Domain `Counter` (from `domain::entity`) is the aggregate root with strong typing.
//! - This module re-exports `CounterResponse` from `packages/contracts/api` for external DTOs.
//! - The service trait operates on domain types internally and maps to contract DTOs at the boundary.

use async_trait::async_trait;
pub use contracts_api::CounterResponse;

use crate::domain::CounterId;

/// Counter operations trait.
#[async_trait]
pub trait CounterService: Send + Sync {
    async fn get_value(&self, tenant_id: &CounterId) -> Result<i64, CounterError>;
    /// Increment with optional idempotency key.
    /// If the key was already processed, returns the cached result.
    async fn increment(
        &self,
        tenant_id: &CounterId,
        idempotency_key: Option<&str>,
    ) -> Result<i64, CounterError>;
    async fn decrement(
        &self,
        tenant_id: &CounterId,
        idempotency_key: Option<&str>,
    ) -> Result<i64, CounterError>;
    async fn reset(
        &self,
        tenant_id: &CounterId,
        idempotency_key: Option<&str>,
    ) -> Result<i64, CounterError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CounterError {
    #[error("Database error: {0}")]
    Database(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Counter not found: {0}")]
    NotFound(String),
    #[error("CAS conflict: counter was modified by another writer")]
    CasConflict,
    /// CAS conflict with version details — used for detailed error responses.
    #[error("CAS conflict: expected version {expected}, actual {actual}")]
    CasConflictWithDetails { expected: i64, actual: i64 },
}
