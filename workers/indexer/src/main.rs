//! Indexer worker — pulls events from sources, transforms, and writes to sinks.
//!
//! Migrated from `servers/indexer/` to `workers/indexer/` per architecture rules.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::sync::RwLock;
use tracing::{info, warn};

mod checkpoint;
mod sinks;
mod sources;
mod transforms;

use checkpoint::SourceCheckpoint;
use sinks::{EventSink, IndexedEvent, MemoryEventSink};
use sources::{EventSource, RawEvent};
use transforms::EventTransform;

/// Indexer error types.
#[derive(Debug, thiserror::Error)]
pub enum IndexerError {
    #[error("Source error: {0}")]
    Source(String),

    #[error("Transform error: {0}")]
    Transform(String),

    #[error("Sink error: {0}")]
    Sink(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Worker state.
struct WorkerState {
    healthy: RwLock<bool>,
    indexed_count: RwLock<u64>,
}

impl WorkerState {
    fn new() -> Self {
        Self {
            healthy: RwLock::new(true),
            indexed_count: RwLock::new(0),
        }
    }

    async fn record_indexed(&self, count: usize) {
        let mut guard = self.indexed_count.write().await;
        *guard += count as u64;
    }
}

/// Health check endpoint.
async fn healthz(state: axum::extract::State<Arc<WorkerState>>) -> axum::Json<serde_json::Value> {
    let indexed = state.indexed_count.read().await;
    axum::Json(serde_json::json!({
        "status": "ok",
        "indexed_count": *indexed,
    }))
}

async fn readyz(state: axum::extract::State<Arc<WorkerState>>) -> axum::Json<serde_json::Value> {
    let healthy = state.healthy.read().await;
    if *healthy {
        axum::Json(serde_json::json!({ "status": "ready" }))
    } else {
        axum::Json(serde_json::json!({ "status": "not ready" }))
    }
}

async fn start_health_server(state: Arc<WorkerState>, addr: SocketAddr) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Indexer health server on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

/// The indexer — coordinates pulling events from sources, transforming, and sinking.
pub struct Indexer {
    sources: Vec<Box<dyn EventSource>>,
    transformers: Vec<Box<dyn EventTransform>>,
    sinks: Vec<Box<dyn EventSink>>,
    checkpoint: SourceCheckpoint,
}

impl Indexer {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            transformers: Vec::new(),
            sinks: Vec::new(),
            checkpoint: SourceCheckpoint::new(),
        }
    }

    pub fn add_source(&mut self, source: Box<dyn EventSource>) {
        self.sources.push(source);
    }

    pub fn add_transformer(&mut self, transformer: Box<dyn EventTransform>) {
        self.transformers.push(transformer);
    }

    pub fn add_sink(&mut self, sink: Box<dyn EventSink>) {
        self.sinks.push(sink);
    }

    /// Run a full indexing cycle.
    pub async fn run_cycle(&self) -> Result<usize, IndexerError> {
        let mut total_indexed = 0;

        // 1. Pull events from all sources
        let mut raw_events = Vec::new();
        for source in &self.sources {
            let cursor = self.checkpoint.get(source.name());
            let events = source.pull_events(cursor.as_deref()).await?;
            raw_events.extend(events);
        }

        if raw_events.is_empty() {
            return Ok(0);
        }

        info!(count = raw_events.len(), "pulled events from sources");

        // 2. Transform raw events to AppEvent
        let mut indexed_events = Vec::new();
        for raw in raw_events {
            for transformer in &self.transformers {
                if transformer.can_transform(&raw) {
                    if let Some(app_event) = transformer.transform(&raw).await? {
                        let event_type = match &app_event {
                            contracts_events::AppEvent::TenantCreated(_) => "tenant.created",
                            contracts_events::AppEvent::TenantMemberAdded(_) => "tenant.member_added",
                            contracts_events::AppEvent::CounterChanged(_) => "counter.changed",
                            contracts_events::AppEvent::ChatMessageSent(_) => "chat.message_sent",
                        };

                        let indexed = IndexedEvent {
                            id: uuid::Uuid::now_v7().to_string(),
                            event_type: event_type.to_string(),
                            source: raw.source.clone(),
                            payload: serde_json::to_string(&app_event)
                                .map_err(|e| IndexerError::Transform(format!("serialize: {e}")))?,
                            indexed_at: chrono::Utc::now().to_rfc3339(),
                        };
                        indexed_events.push(indexed);
                    }
                    break;
                }
            }
        }

        if indexed_events.is_empty() {
            return Ok(0);
        }

        // 3. Write to all sinks
        for event in &indexed_events {
            for sink in &self.sinks {
                sink.write(event).await?;
                total_indexed += 1;
            }
        }

        // 4. Update checkpoints
        for source in &self.sources {
            // In a real implementation, the source would return a cursor
            // For now, we use a placeholder
            self.checkpoint.update(source.name(), "latest".to_string());
        }

        info!(count = total_indexed, "indexing cycle complete");
        Ok(total_indexed)
    }
}

impl Default for Indexer {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "indexer_worker=info".into()),
        )
        .init();

    info!("Indexer worker starting");

    let state = Arc::new(WorkerState::new());

    // Health server
    let health_addr: SocketAddr = "0.0.0.0:3031".parse()?;
    let health_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = start_health_server(health_state, health_addr).await {
            warn!(error = %e, "health server failed");
        }
    });

    // Build indexer with stub sources/sinks
    let mut indexer = Indexer::new();
    indexer.add_source(Box::new(sources::MemoryEventSource::new(Vec::new())));
    indexer.add_transformer(Box::new(transforms::PassthroughTransform));
    indexer.add_sink(Box::new(sinks::MemoryEventSink::new()));

    info!("Indexer worker running (stub mode)");

    // Main loop
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
    loop {
        interval.tick().await;
        match indexer.run_cycle().await {
            Ok(count) => {
                state.record_indexed(count).await;
            }
            Err(e) => {
                warn!(error = %e, "indexing cycle failed");
            }
        }
    }
}
