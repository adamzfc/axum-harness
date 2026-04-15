//! Domain-level error types for the Counter aggregate.

use thiserror::Error;

/// All error variants that can occur within the Counter domain.
///
/// ## Design rules
/// - Never expose infrastructure details (SQL, HTTP, etc.) at this level
/// - Each variant must carry enough context for the application layer to decide
/// - Variants must be exhaustive — no catch-all `Other` variant
#[derive(Debug, Error)]
pub enum CounterDomainError {
    #[error("counter not found: {0}")]
    NotFound(String),

    #[error("counter operation failed: {reason}")]
    OperationFailed { reason: String },

    /// CAS conflict — another writer modified the counter between load and update.
    /// The application layer should retry with the latest version.
    #[error(
        "CAS conflict: counter was modified concurrently (expected version {expected}, actual {actual})"
    )]
    CasConflict { expected: i64, actual: i64 },
}
