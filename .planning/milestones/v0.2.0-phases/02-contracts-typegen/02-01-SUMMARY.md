---
phase: 02-contracts-typegen
plan: 01
subsystem: contracts
tags: [rust, ts-rs, typegen, contracts, moon, serde, utoipa]

# Dependency graph
requires:
  - phase: 01-repo-structure-toolchain
    provides: Cargo workspace, moon.yml task graph, directory scaffold
provides:
  - Three compilable contracts crates (api, auth, events) with ts-rs derive macros
  - Working typegen pipeline: cargo test → .ts files in packages/contracts/generated/
  - Moon tasks: repo:typegen (generate) and repo:contracts-check (drift detection)
  - Initial DTOs: HealthResponse, InitTenantRequest/Response, TokenPair, OAuthCallback, UserSession, TenantCreated, TenantMemberAdded
affects: [phase-04-minimal-features, servers-api-routes, frontend-types]

# Tech tracking
tech-stack:
  added: [ts-rs 10, utoipa 5]
  patterns: [Rust DTO → TS auto-generation via ts-rs derive, moon task for typegen+drift-check]

key-files:
  created:
    - packages/contracts/auth/Cargo.toml
    - packages/contracts/auth/src/lib.rs
    - packages/contracts/events/Cargo.toml
    - packages/contracts/events/src/lib.rs
    - packages/contracts/generated/.gitignore
    - apps/client/web/app/src/lib/generated/.gitkeep
  modified:
    - Cargo.toml (workspace members + deps: contracts_auth, contracts_events, ts-rs, utoipa)
    - packages/contracts/api/Cargo.toml (added ts-rs, utoipa, serde, validator deps)
    - packages/contracts/api/src/lib.rs (DTOs with ts-rs + utoipa derives)
    - moon.yml (repo:typegen and repo:contracts-check implementations)

key-decisions:
  - "ts-rs chosen as Rust→TS codegen tool (mature, serde-compatible, derive-based)"
  - "All cross-boundary DTOs use #[derive(TS)] + #[ts(export)] for automatic generation"
  - "Contracts split into api/auth/events crates per blueprint alignment (D-03)"
  - "Typegen pipeline: cargo test generates to per-crate bindings/, then copies to generated/"
  - "Drift check via git diff --exit-code on generated/ directory (D-08)"
  - "utoipa coexists with ts-rs on same struct — OpenAPI and TS types from single source"

patterns-established:
  - "Rust DTO pattern: #[derive(Serialize, Deserialize, ToSchema, TS)] + #[ts(export, export_to = \"subdir/\")]"
  - "Typegen pipeline: cargo test → per-crate bindings/ → cp to shared generated/"
  - "Moon task pattern: command block with cargo test + cp + echo, inputs scoped to contracts"
  - "Drift detection: moon run repo:typegen then git diff --exit-code on generated/"

requirements-completed: [CONTRACT-01, CONTRACT-02]

# Metrics
duration: 15min
completed: 2026-04-01
---

# Phase 2 Plan 1: Contracts/typegen Summary

**Three contracts crates (api, auth, events) with ts-rs derive macros, 8 auto-generated TypeScript DTOs, and moon typegen+drift-check pipeline**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-04-01T23:30:00Z
- **Completed:** 2026-04-01T23:45:00Z
- **Tasks:** 2
- **Files modified:** 14

## Accomplishments
- Created `contracts_auth` and `contracts_events` crates with initial DTOs (TokenPair, OAuthCallback, UserSession, TenantCreated, TenantMemberAdded)
- Populated `contracts_api` with ts-rs + utoipa DTOs mirroring server route types (HealthResponse, InitTenantRequest/Response)
- Implemented typegen pipeline: `cargo test` generates .ts files → copied to `packages/contracts/generated/`
- Implemented drift detection: `repo:contracts-check` runs typegen then `git diff --exit-code`
- Added ts-rs 10 and utoipa 5 to workspace dependencies

## Task Commits

1. **Task 1: Set up contracts crates with ts-rs and initial DTOs** - `f2fe128` (feat)
2. **Task 2: Implement typegen pipeline and moon tasks** - `057cf38` (feat)

## Files Created/Modified
- `Cargo.toml` — Added workspace members (auth, events), deps (ts-rs, utoipa, contracts_auth, contracts_events)
- `packages/contracts/api/Cargo.toml` — Added serde, utoipa, ts-rs, validator deps
- `packages/contracts/api/src/lib.rs` — HealthResponse, InitTenantRequest, InitTenantResponse with ts-rs + utoipa
- `packages/contracts/auth/Cargo.toml` — New crate with serde, utoipa, ts-rs deps
- `packages/contracts/auth/src/lib.rs` — TokenPair, OAuthCallback, UserSession
- `packages/contracts/events/Cargo.toml` — New crate with serde, utoipa, ts-rs deps
- `packages/contracts/events/src/lib.rs` — TenantCreated, TenantMemberAdded
- `moon.yml` — Working repo:typegen (cargo test + copy) and repo:contracts-check (drift detection)
- `packages/contracts/generated/.gitignore` — Generated types exclusion with self-inclusion
- `apps/client/web/app/src/lib/generated/.gitkeep` — Frontend import placeholder
- `.gitignore` — Added `packages/contracts/*/bindings/` for intermediate ts-rs output

## Decisions Made
- ts-rs 10 selected over alternatives (most mature, serde-compatible, simple derive macro)
- Contracts split into 3 separate crates (api, auth, events) per blueprint alignment — each concern has its own namespace
- Typegen outputs to per-crate `bindings/` then copies to shared `generated/` — ts-rs exports relative to CARGO_MANIFEST_DIR, so copy step needed for centralized output
- utoipa and ts-rs coexist on same struct — OpenAPI schemas and TS types from single source of truth

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered
- ts-rs `#[ts(export)]` generates during test execution, not just compilation — added `#[cfg(test)]` modules with `Type::export().unwrap()` calls to trigger generation
- Generated files land in per-crate `bindings/` not shared `generated/` — added copy step to typegen pipeline

## Next Phase Readiness
- Contracts layer established as single truth source for cross-boundary types
- Typegen pipeline operational: `cargo test` → 8 `.ts` files in `packages/contracts/generated/`
- Ready for Phase 2 Plan 2 (if exists) or Phase 4 to consume contracts in server routes and frontend

---

*Phase: 02-contracts-typegen*
*Completed: 2026-04-01*

## Self-Check: PASSED

- [x] All files exist (10/10 checked)
- [x] All commits exist (f2fe128, 057cf38 verified)
