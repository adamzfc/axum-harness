//! Counter service — ports (external dependency abstractions).
//!
//! This module defines **what** the counter service needs from the outside world,
//! not **how** those needs are fulfilled. Implementations live in adapters.

mod repository;

pub use repository::*;
