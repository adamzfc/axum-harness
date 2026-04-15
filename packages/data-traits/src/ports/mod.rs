//! Port types and cross-crate utilities.
//!
//! TenantId is NOT re-exported here — consumers should import directly from kernel::TenantId
pub mod lib_sql;
pub mod surreal_db;
