//! Counter domain — pure entities, value objects, and invariants.
//!
//! This module contains zero external dependencies.
//! All types here represent the "truth" about what a Counter is.

mod entity;
mod error;

pub use entity::*;
pub use error::*;
