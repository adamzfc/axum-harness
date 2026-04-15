//! Counter service reference library.
//!
//! This is the smallest end-to-end sample in the repo and the primary copy target
//! for new business service skeletons.
//!
//! Target reference layout:
//! - `domain/` → aggregate rules and invariants
//! - `application/` → command/query orchestration
//! - `ports/` → external dependency abstractions
//! - `events/` → service-local event intent
//! - `policies/` → policy placeholders and rule hooks
//! - `contracts/` → DTO re-exports from shared contracts
//!
//! `infrastructure/` remains temporarily because existing server and desktop
//! composition roots still instantiate `LibSqlCounterRepository` from here.

// ── Core layers ──
pub mod application;
pub mod contracts;
pub mod domain;
pub mod events;
pub mod policies;
pub mod ports;

pub mod infrastructure;
