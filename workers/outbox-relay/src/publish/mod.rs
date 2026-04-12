//! Event publisher — publishes outbox entries to the event bus.

use async_trait::async_trait;
use contracts_events::AppEvent;
use event_bus::ports::{EventBus, EventEnvelope};
use tracing::{debug, warn};

use crate::polling::PendingOutboxEntry;

/// Error type for publishing.
#[derive(Debug, thiserror::Error)]
pub enum PublishError {
    #[error("Failed to deserialize event: {0}")]
    DeserializeError(String),

    #[error("Failed to publish to event bus: {0}")]
    BusError(String),
}

/// Publishes outbox entries to the event bus.
pub struct OutboxPublisher<E: EventBus> {
    event_bus: E,
}

impl<E: EventBus> OutboxPublisher<E> {
    pub fn new(event_bus: E) -> Self {
        Self { event_bus }
    }

    /// Publish a single outbox entry to the event bus.
    pub async fn publish(
        &self,
        entry: &PendingOutboxEntry,
    ) -> Result<(), PublishError> {
        let event: AppEvent = serde_json::from_str(&entry.payload)
            .map_err(|e| PublishError::DeserializeError(e.to_string()))?;

        let envelope = EventEnvelope::new(event, &entry.source_service);

        self.event_bus
            .publish(envelope)
            .await
            .map_err(|e| PublishError::BusError(e.to_string()))?;

        debug!(entry_id = %entry.id, "published outbox entry to event bus");
        Ok(())
    }

    /// Publish a batch of outbox entries, returning successes and failures.
    pub async fn publish_batch(
        &self,
        entries: &[PendingOutboxEntry],
    ) -> (Vec<String>, Vec<(String, PublishError)>) {
        let mut successes = Vec::new();
        let mut failures = Vec::new();

        for entry in entries {
            match self.publish(entry).await {
                Ok(()) => successes.push(entry.id.clone()),
                Err(e) => {
                    warn!(entry_id = %entry.id, error = %e, "failed to publish outbox entry");
                    failures.push((entry.id.clone(), e));
                }
            }
        }

        (successes, failures)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use event_bus::adapters::memory_bus::InMemoryEventBus;

    fn test_entry(id: &str, payload: &str) -> PendingOutboxEntry {
        PendingOutboxEntry {
            id: id.to_string(),
            sequence: 1,
            event_type: "counter.changed".to_string(),
            payload: payload.to_string(),
            source_service: "counter-service".to_string(),
            retry_count: 0,
        }
    }

    #[tokio::test]
    async fn publishes_valid_event() {
        let bus = InMemoryEventBus::new();
        let publisher = OutboxPublisher::new(bus);

        // CounterChanged event with valid JSON (AppEvent uses internally tagged format)
        let event = contracts_events::AppEvent::CounterChanged(contracts_events::CounterChanged {
            tenant_id: "test-tenant".to_string(),
            new_value: 42,
            delta: 1,
        });
        let payload = serde_json::to_string(&event).unwrap();

        let entry = test_entry("entry-1", &payload);
        let result = publisher.publish(&entry).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn returns_error_for_invalid_json() {
        let bus = InMemoryEventBus::new();
        let publisher = OutboxPublisher::new(bus);

        let entry = test_entry("entry-1", "not valid json");
        let result = publisher.publish(&entry).await;
        assert!(matches!(result, Err(PublishError::DeserializeError(_))));
    }
}
