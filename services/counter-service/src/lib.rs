//! Counter Service — Golden Module 🏆
//!
//! This is the reference implementation for all services in this project.
//! New services should copy this structure and conventions.
//!
//! ## Architecture (Clean Architecture / Hexagonal)
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │  interfaces/  (future: gRPC, HTTP routes)       │  ← Outer: protocol adapters
//! ├─────────────────────────────────────────────────┤
//! │  infrastructure/ (LibSqlCounterRepository)      │  ← Outer: storage adapters
//! ├─────────────────────────────────────────────────┤
//! │  application/   (TenantScopedCounterService)    │  ← Inner: use case orchestration
//! ├─────────────────────────────────────────────────┤
//! │  ports/         (CounterRepository trait)       │  ← Inner: external dependency abstract
//! ├─────────────────────────────────────────────────┤
//! │  domain/        (Counter, CounterId, errors)    │  ← Core: entities & invariants
//! └─────────────────────────────────────────────────┘
//!
//!  contracts/     (DTO re-exports from packages/contracts/)
//!  sync/          (OfflineFirst sync strategies)
//! ```
//!
//! ## Dependency rules
//! - `domain/` → zero external dependencies (pure Rust types)
//! - `ports/` → only `domain/` + async-trait
//! - `application/` → `domain/` + `ports/` + feature traits
//! - `infrastructure/` → `ports/` + specific storage crates
//! - `interfaces/` → `application/` + HTTP/gRPC frameworks
//!
//! ## Feature flags
//! - `trait-only` — exports only trait definitions (for BFF compile-time dependency)
//!
//! ## Migration
//! The `COUNTER_MIGRATION` constant in `application::service` contains the SQL
//! DDL. Run this at startup from the composition root:
//!
//! ```ignore
//! use counter_service::application::service::COUNTER_MIGRATION;
//! db.execute(COUNTER_MIGRATION, vec![]).await?;
//! ```

// ── Core layers ──
pub mod application;
pub mod contracts;
pub mod domain;
pub mod ports;
pub mod sync;

// ── Outer layers ──
pub mod infrastructure;
pub mod interfaces;
