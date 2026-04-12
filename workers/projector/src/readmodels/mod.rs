//! Read model builders — materialized views built from event streams.

use async_trait::async_trait;

use crate::ProjectorError;

/// A read model — a materialized view updated from events.
#[async_trait]
pub trait ReadModel: Send + Sync {
    /// Name of this read model.
    fn name(&self) -> &str;

    /// Apply an update from a consumer.
    async fn apply_update(&self, update: &str) -> Result<(), ProjectorError>;
}

/// In-memory stub read model for testing.
pub struct MemoryReadModel {
    pub updates: tokio::sync::Mutex<Vec<String>>,
}

impl MemoryReadModel {
    pub fn new() -> Self {
        Self {
            updates: tokio::sync::Mutex::new(Vec::new()),
        }
    }
}

impl Default for MemoryReadModel {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReadModel for MemoryReadModel {
    fn name(&self) -> &str {
        "memory-read-model"
    }

    async fn apply_update(&self, update: &str) -> Result<(), ProjectorError> {
        self.updates.lock().await.push(update.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn read_model_collects_updates() {
        let model = MemoryReadModel::new();
        model.apply_update("update-1").await.unwrap();
        model.apply_update("update-2").await.unwrap();

        let updates = model.updates.lock().await;
        assert_eq!(updates.len(), 2);
    }
}
