//! Counter-domain event markers.
//!
//! This module re-exports the canonical event schemas from `packages/contracts/events`.
//! The service-local event intent is documented here; the actual type definitions
//! live in the contracts package to ensure a single source of truth.
//!
//! ## Published Events
//!
//! | Event | Schema Location | Description |
//! |-------|----------------|-------------|
//! | `counter.changed` | `packages/contracts/events/src/lib.rs::CounterChanged` | Emitted after any successful counter mutation (increment/decrement/reset) |
//!
//! ## Dedupe Rule
//! `tenant_id + counter_key + version`
//!
//! ## Ordering Scope
//! per-tenant
//!
//! ## Replay
//! Events are replayable with a 30-day retention policy.
//! Backward compatibility policy applies to schema changes.

pub use contracts_events::CounterChanged;
