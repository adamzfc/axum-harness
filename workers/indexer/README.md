# Indexer Worker

Pulls events from various protocol sources, normalizes them to business DTOs, and writes to storage sinks.

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│ Sources     │────▶│ Transforms   │────▶│ Sinks       │
│ (sources/)  │     │ (transforms/)│     │ (sinks/)    │
└─────────────┘     └──────────────┘     └─────────────┘
       │                                        │
       ▼                                        ▼
┌─────────────────────────────┐    ┌────────────────────────────┐
│ Source Checkpoint            │    │ Indexed events (Turso)     │
│ (checkpoint/)                │    │                            │
└─────────────────────────────┘    └────────────────────────────┘
```

## Pattern

1. **Pull** events from all registered sources (with per-source cursors)
2. **Transform** raw protocol events → `AppEvent` business DTOs
3. **Sink** to storage (Turso for query)
4. **Checkpoint** each source independently

## Health Checks

- `GET /healthz` — Liveness + indexed count
- `GET /readyz` — Readiness probe

## Current State

- ✅ Source/transform/sink pipeline
- ✅ Per-source checkpoint tracking
- ✅ Health check endpoints
- ⬜ Real protocol sources (stub MemoryEventSource for now)
- ⬜ Real Turso sink (stub MemoryEventSink for now)
