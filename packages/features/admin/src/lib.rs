//! Admin feature — dashboard statistics from real data.
//!
//! Hexagonal boundary: re-exports DTOs from contracts_api,
//! defines AdminService trait only.

use async_trait::async_trait;
pub use contracts_api::AdminDashboardStats;

/// Admin operations trait.
#[async_trait]
pub trait AdminService: Send + Sync {
    async fn get_dashboard_stats(&self) -> Result<AdminDashboardStats, AdminError>;
}

#[derive(Debug, thiserror::Error)]
pub enum AdminError {
    #[error("Database error: {0}")]
    Database(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Counter error: {0}")]
    Counter(String),
}
