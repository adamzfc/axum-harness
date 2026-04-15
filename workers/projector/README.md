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

## Required Strategies

1. Replay: projector must support rebuilding from replayable source events
2. Rebuildability: read models are disposable and must not become the source of truth
3. Checkpoint: persist the last safe event offset before advancing
4. Recovery order: restore checkpoint, replay if required, then resume live consumption
5. Lag semantics: projection lag is observable and acceptable within declared SLOs

## Health Checks

- `GET /healthz` — Liveness + projected count
- `GET /readyz` — Readiness probe

## Current State

- ✅ Consumer/read model pipeline
- ✅ Projection checkpointing
- ⬜ Real event bus subscription (stub for now)
- ⬜ Real database-backed read models (stub MemoryReadModel)
