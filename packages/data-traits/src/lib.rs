//! data-traits crate — Port trait definitions for database backends.
//!
//! Per D-05/D-06: trait-per-DB pattern. Each database backend gets its own Port trait.
//! This crate contains only trait definitions with zero vendor dependencies.
//! Implementations live in data-adapters crates (Turso, SurrealDB, etc.).
pub mod ports;
