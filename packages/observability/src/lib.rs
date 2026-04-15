//! Observability layer — aggregation crate for telemetry adapters.
//!
//! Re-exports from adapter subcrates:
//! - `opentelemetry` — OpenTelemetry integration
//! - `tracing` — tracing-subscriber integration

// Re-export adapter crates
pub use adapter_telemetry_otel as opentelemetry;
pub use adapter_telemetry_tracing as tracing;
