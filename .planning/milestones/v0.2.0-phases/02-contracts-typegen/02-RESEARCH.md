# Phase 02: Contracts/typegen — Research

**Date:** 2026-04-01
**Status:** Complete

## Summary

Rust→TS type generation via ts-rs is the clear choice. The project already has `packages/contracts/api/` as a workspace member with utoipa for OpenAPI. ts-rs and utoipa coexist on the same struct (dual derive). The pipeline is: derive TS on DTOs → ts-rs exports to `generated/` → moon task runs typegen → drift check via `git diff --exit-code`.

## Key Findings

### ts-rs Integration
- ts-rs v10+ (2026) is mature, serde-compatible, uses `#[derive(TS)]` + `#[ts(export)]`
- Coexists with utoipa: same struct can have `#[derive(Serialize, Deserialize, ToSchema, TS)]`
- Outputs per-struct `.ts` files to `bindings/` directory by default
- Supports `#[ts(export_to = "path")]` for custom output locations
- Cargo feature `ts-rs/chrono-impl` and `ts-rs/uuid-impl` for common types

### Crate Structure
- Three concern-separated modules under `packages/contracts/`:
  - `api` — route-level DTOs (request/response). Already workspace member.
  - `auth` — auth types (tokens, sessions). Currently .gitkeep only.
  - `events` — domain event payloads. Currently .gitkeep only.
- Decision: Use separate crates (not modules within api) for clean dependency boundaries
- Each crate needs `Cargo.toml` with ts-rs + serde + utoipa dependencies

### Migration Targets
- `servers/api/src/routes/tenant.rs`: `InitTenantRequest` (derive: Deserialize, Serialize, Validate, ToSchema), `InitTenantResponse` (derive: Serialize, ToSchema)
- `servers/api/src/routes/health.rs`: `HealthResponse` (derive: ToSchema only, no Serialize)
- Both already use serde + utoipa → adding ts-rs is additive

### Pipeline Design
- `repo:typegen`: `cargo test -p contracts_api -p contracts_auth -p contracts_events` (ts-rs exports during tests)
- Output: `packages/contracts/generated/{api,auth,events}/*.ts`
- Frontend sync: symlink or copy to `apps/client/web/app/src/lib/generated/`
- `repo:contracts-check`: run typegen → `git diff --exit-code packages/contracts/generated/` → fail if drift

### Open Questions (resolved)
- ts-rs version: latest stable (v10+)
- Auth/events as separate crates vs modules: separate crates (cleaner deps)
- Frontend sync mechanism: post-typegen copy script (more portable than symlink on Windows)
- utoipa coexistence: confirmed compatible, dual derive on same struct

## Technical Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Typegen library | ts-rs | Most mature Rust→TS in 2026, serde native |
| Auth/events structure | Separate crates | Clean dependency graph, independent versioning |
| Generated output | `packages/contracts/generated/` | Central location, all crates write here |
| Frontend sync | Post-typegen copy | Cross-platform, no symlink issues |
| Drift check | `git diff --exit-code` | Simple, no extra tooling needed |
| moon typegen task | `cargo test` (ts-rs exports) | Standard ts-rs workflow |

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| ts-rs test-export timing | Types only generated during `cargo test` | Use `cargo test` in typegen task, not `cargo build` |
| Generated files in git | Large diffs, merge conflicts | .gitignore generated/ at source, commit only final output |
| utoipa/ts-rs field divergence | Types drift between OpenAPI and TS | Same struct, dual derive — always in sync |
