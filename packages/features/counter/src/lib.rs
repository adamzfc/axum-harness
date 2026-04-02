//! Counter feature — increment/decrement/reset with persistence.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Counter state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter {
    pub id: String,
    pub value: i64,
    pub updated_at: String,
}

/// Counter operations trait.
#[async_trait]
pub trait CounterService: Send + Sync {
    async fn get_value(&self) -> Result<i64, CounterError>;
    async fn increment(&self) -> Result<i64, CounterError>;
    async fn decrement(&self) -> Result<i64, CounterError>;
    async fn reset(&self) -> Result<i64, CounterError>;
}

#[derive(Debug, thiserror::Error)]
pub enum CounterError {
    #[error("Database error: {0}")]
    Database(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("Counter not found")]
    NotFound,
}
