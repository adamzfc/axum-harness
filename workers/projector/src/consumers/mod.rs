//! Event consumers — consume events from the event bus and build read models.

use async_trait::async_trait;
use contracts_events::AppEvent;
use event_bus::ports::EventEnvelope;

use crate::ProjectorError;

/// Abstract event consumer for the projector.
#[async_trait]
pub trait EventConsumer: Send + Sync {
    /// Name of this consumer.
    fn name(&self) -> &str;

    /// Check if this consumer is interested in the event.
    fn is_interested(&self, event: &AppEvent) -> bool;

    /// Process the event and produce an optional read model update.
    async fn consume(&self, envelope: &EventEnvelope) -> Result<Option<String>, ProjectorError>;
}

/// Stub consumer for testing.
pub struct LoggingConsumer;

#[async_trait]
impl EventConsumer for LoggingConsumer {
    fn name(&self) -> &str {
        "logging"
    }

    fn is_interested(&self, _event: &AppEvent) -> bool {
        true
    }

    async fn consume(&self, envelope: &EventEnvelope) -> Result<Option<String>, ProjectorError> {
        Ok(Some(format!(
            "Consumed event from {} at {}",
            envelope.source_service,
            chrono::Utc::now().to_rfc3339()
        )))
    }
}
