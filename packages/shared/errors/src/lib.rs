//! shared-errors — Re-exports from contracts_errors.
//!
//! This crate exists so that code in `packages/` can depend on
//! `shared-errors` without directly depending on `contracts/`,
//! maintaining clean layer boundaries.

pub use contracts_errors::*;
