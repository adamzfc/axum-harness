//! Counter contracts — stable DTO definitions.
//!
//! This module re-exports from `packages/contracts/` (the single source of truth)
//! and adds any counter-specific contract types that don't belong in shared contracts.

pub mod service;

// Re-export HTTP DTOs from the single source of truth.
// BFF and other consumers should prefer these re-exports over direct
// contracts_api imports to maintain a clean dependency boundary.
pub use contracts_api::CounterResponse;
