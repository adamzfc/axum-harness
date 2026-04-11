//! Kernel — base types for the entire system.
//!
//! This crate defines the foundational types used across all layers:
//! - `TenantId` — tenant identifier (extracted from JWT)
//! - `UserId` — user identifier
//! - `AppError` — unified error type
//! - `Cursor` — pagination cursor

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ── TenantId ──────────────────────────────────────────────────

/// Tenant identifier — extracted from JWT `sub` claim.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(pub String);

impl TenantId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── UserId ────────────────────────────────────────────────────

/// User identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub String);

impl UserId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── Cursor ────────────────────────────────────────────────────

/// Pagination cursor (timestamp-based for keyset pagination).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor(pub String);

// ── AppError ──────────────────────────────────────────────────

/// Unified application error type.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("tenant not found: {0}")]
    TenantNotFound(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("validation error: {0}")]
    Validation(String),
}
