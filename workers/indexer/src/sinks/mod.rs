//! Event sinks — write indexed events to storage.

use async_trait::async_trait;
use contracts_events::AppEvent;

use crate::IndexerError;

/// Indexed event record — stored for query.
#[derive(Debug, Clone)]
pub struct IndexedEvent {
    pub id: String,
    pub event_type: String,
    pub source: String,
    pub payload: String, // JSON-serialized AppEvent
    pub indexed_at: String,
}

/// Event sink trait — writes events to a storage layer.
#[async_trait]
pub trait EventSink: Send + Sync {
    /// Name of this sink (e.g., "turso-events").
    fn name(&self) -> &str;

    /// Write an event to the sink.
    async fn write(&self, event: &IndexedEvent) -> Result<(), IndexerError>;
}

/// In-memory stub sink for testing — collects events.
pub struct MemoryEventSink {
    pub events: tokio::sync::Mutex<Vec<IndexedEvent>>,
}

impl MemoryEventSink {
    pub fn new() -> Self {
        Self {
            events: tokio::sync::Mutex::new(Vec::new()),
        }
    }
}

impl Default for MemoryEventSink {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventSink for MemoryEventSink {
    fn name(&self) -> &str {
        "memory-sink"
    }

    async fn write(&self, event: &IndexedEvent) -> Result<(), IndexerError> {
        self.events.lock().await.push(event.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn memory_sink_collects_events() {
        let sink = MemoryEventSink::new();
        let event = IndexedEvent {
            id: "evt-1".to_string(),
            event_type: "counter.changed".to_string(),
            source: "test".to_string(),
            payload: "{}".to_string(),
            indexed_at: "now".to_string(),
        };

        sink.write(&event).await.unwrap();
        let events = sink.events.lock().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, "evt-1");
    }
}
