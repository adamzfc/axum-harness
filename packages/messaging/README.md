# Event Bus — Unified Outbox & Inter-Service Communication

> `packages/messaging` is the **single event persistence truth source** for all services.
> Every service writes to the unified `event_outbox` table owned by this crate.

## Architecture

```text
┌────────────────────────────────────────────┐
│  ports/         (EventBus trait)            │  ← Services depend on this
├────────────────────────────────────────────┤
│  adapters/      (InMemoryEventBus)         │  ← In-process
│                 (NatsEventBus)              │  ← Distributed
├────────────────────────────────────────────┤
│  outbox/        (event_outbox schema +     │  ← Unified outbox truth source
│                  OutboxEntry +              │
│                  OutboxPublisher)           │
└────────────────────────────────────────────┘
```

## Key Design

- `event_outbox` is the **only** event persistence table — no per-service private outbox tables
- Schema: `sequence INTEGER PRIMARY KEY AUTOINCREMENT` + `event_id TEXT UNIQUE` (UUID v7)
- `status` / `retry_count` / `published_at` track delivery state
- outbox-relay worker reads from this table and publishes to EventBus + PubSub

## Ownership

- Schema definition: `src/outbox/outbox_entry.rs`
- Publisher logic: `src/outbox/outbox_publisher.rs`
- Event types: `packages/contracts/events/`

## Feature Flags

- `memory` (default) — in-memory event bus via tokio broadcast channels
- `nats` (future) — NATS JetStream implementation for production
