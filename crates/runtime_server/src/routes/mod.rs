//! Route module barrel — all feature route modules exported here.

pub mod health;
pub mod tenant;

use crate::state::AppState;
use axum::Router;

/// Merge all route modules into a single router.
pub fn router() -> Router<AppState> {
    Router::<AppState>::new().merge(health::router())
}
