//! adapter-telemetry-otel — OpenTelemetry integration for distributed tracing.
//!
//! Provides initialization helpers for OpenTelemetry SDK with OTLP exporter.
//! No-op until initialized by the composition layer (servers/bff or apps/client/native).

/// Initialize OpenTelemetry tracing with OTLP exporter.
/// Returns a tracer provider that should be kept alive.
pub fn init_otel_tracing(_service_name: &str) -> Result<(), String> {
    // Stub: actual initialization requires OTLP endpoint configuration
    // This will be implemented when OpenObserve/Vector integration is wired up.
    Ok(())
}
