//! Tenant initialization endpoints — placeholder.
//!
//! Full implementation in Plan 03. This file exists so `pub mod tenant;`
//! in routes/mod.rs compiles during Plan 02.

use crate::state::AppState;
use axum::Router;

/// Placeholder router — Plan 03 replaces with real tenant init endpoint.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
}
