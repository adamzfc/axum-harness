# Telemetry Adapters

External protocol translators for observability data.

## Structure

```
telemetry/
├── otel/    # OpenTelemetry SDK integration (traces + metrics)
└── tracing/ # tracing-subscriber integration (structured logging)
```

## Status

⚠️ **Stub** — These crates provide the adapter layer for telemetry.
Actual initialization happens in the composition layers (`servers/bff/web-bff/`, `servers/bff/admin-bff/`, `apps/desktop/src-tauri/`).

## Design

Per the architecture principle **"adapters are thin, core is stable"**:
- These crates contain only protocol-specific initialization code
- No business logic
- Services depend on `tracing` macros, not on specific SDKs
- Swapping observability backends (e.g., OpenObserve → Datadog) only requires changing the adapter, not service code
