# Projector Worker

Builds read models from event streams by consuming events and updating materialized views.

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│ Consumers   │────▶│ Read Models  │────▶│ Checkpoint  │
│ (consumers/)│     │ (readmodels/)│     │ (checkpoint/)│
└─────────────┘     └──────────────┘     └─────────────┘
```

## Pattern

1. **Consume** events from the event bus (interested consumers only)
2. **Project** onto read models (materialized views)
3. **Checkpoint** the last processed event sequence

## Health Checks

- `GET /healthz` — Liveness + projected count
- `GET /readyz` — Readiness probe

## Current State

- ✅ Consumer/read model pipeline
- ✅ Projection checkpointing
- ⬜ Real event bus subscription (stub for now)
- ⬜ Real database-backed read models (stub MemoryReadModel)
