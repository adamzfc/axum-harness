# Outbox Relay Worker

Reliable event delivery worker that polls the outbox table, publishes events to the event bus, and tracks checkpoints/deduplication.

## Architecture

```
┌─────────────┐     ┌──────────┐     ┌─────────────┐
│ OutboxPoller│────▶│ Dedup    │────▶│ Publisher   │
│ (polling/)  │     │ (dedupe/)│     │ (publish/)  │
└─────────────┘     └──────────┘     └─────────────┘
       │                                    │
       ▼                                    ▼
┌─────────────┐                     ┌─────────────┐
│ Checkpoint  │                     │ EventBus    │
│ (checkpoint/)│                     │             │
└─────────────┘                     └─────────────┘
```

## Pattern

1. **Poll** the outbox table for pending entries
2. **Deduplicate** using an in-memory LRU cache
3. **Publish** to the event bus
4. **Checkpoint** the last processed sequence number

## Required Strategies

1. Idempotency: dedupe by event identity before publish
2. Retry: tolerate at-least-once delivery and transient publish failures
3. Checkpoint: persist relay progress so restart resumes from last safe offset
4. Recovery order: restore checkpoint, then resume polling, then continue publish attempts

## Health Checks

- `GET /healthz` — Liveness probe (always returns ok unless panicked)
- `GET /readyz` — Readiness probe (returns ready when initialized)

## Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| `POLL_INTERVAL_SECS` | `5` | Seconds between poll cycles |
| `BATCH_SIZE` | `100` | Max entries per batch |
| `HEALTH_PORT` | `3030` | Health check server port |

## Current State

This is a **skeleton implementation** with:
- ✅ Polling, dedup, checkpoint, publish modules
- ✅ In-memory event bus (production would use NATS)
- ✅ Health check endpoints
- ⬜ Real database reader (stub MemoryOutboxReader for now)
- ⬜ NATS event bus integration
