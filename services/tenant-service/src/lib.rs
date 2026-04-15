//! Tenant service reference library.
//!
//! This is the multi-entity and workflow-oriented reference service.
//! The target skeleton keeps domain semantics in `model.yaml`, while source code
//! remains focused on domain/application/ports/contracts/events/policies.
//!
//! `infrastructure/` remains temporarily because existing composition roots still
//! construct repository adapters from this crate.

pub mod application;
pub mod contracts;
pub mod domain;
pub mod events;
pub mod infrastructure;
pub mod policies;
pub mod ports;
