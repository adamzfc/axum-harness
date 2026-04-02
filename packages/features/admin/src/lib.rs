//! Admin feature — dashboard statistics from real data.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Dashboard statistics aggregated from real data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub tenant_count: i64,
    pub counter_value: i64,
    pub last_login: Option<String>,
    pub app_version: String,
}

/// Admin operations trait.
#[async_trait]
pub trait AdminService: Send + Sync {
    async fn get_dashboard_stats(&self) -> Result<DashboardStats, AdminError>;
}

#[derive(Debug, thiserror::Error)]
pub enum AdminError {
    #[error("Database error: {0}")]
    Database(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Counter error: {0}")]
    Counter(String),
}
