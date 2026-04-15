//! data crate — Aggregation facade for database layer.
//!
//! Re-exports port trait definitions from data-traits.
//! Adapter implementations live in data-adapters crates (Turso, SurrealDB, etc.).
//!
//! Consumers can depend on either:
//! - `data` (this crate) for the re-exported ports module
//! - `data-traits` directly for port definitions only
pub use data_traits::ports;
