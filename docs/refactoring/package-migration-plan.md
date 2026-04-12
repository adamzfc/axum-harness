# Package Migration Plan — Phase 2

> **Date**: 2026-04-12
> **Agent**: agent-phase-2
> **Scope**: Align packages/ with ARCHITECTURE.md Section 3.8

---

## Strategy: Incremental, Verifiable, Low-Risk

Based on dependency analysis, we'll migrate in this order (each step independently verifiable):

### Step 1: Move kernel (LOW risk)
- `packages/core/kernel` → `packages/kernel`
- Update root Cargo.toml workspace members
- Update root Cargo.toml workspace.dependencies path
- Verify: `cargo check --workspace`

### Step 2: Move platform (TRIVIAL risk)
- `packages/core/platform` → `packages/platform`
- Update root Cargo.toml workspace members
- Update root Cargo.toml workspace.dependencies path
- Verify: `cargo check --workspace`

### Step 3: Restructure contracts (MEDIUM risk)
- Current: `packages/contracts/{api, auth, events, errors}`
- Target: `packages/contracts/{http, events, rpc, jsonschema, error-codes, compat, sdk-gen}`
- Map current to target:
  - `contracts/api` → `contracts/http` (HTTP contracts)
  - `contracts/events` → `contracts/events` (stays, just move)
  - `contracts/auth` → SPLIT: OAuth types to `contracts/http/auth/`, RPC types to `contracts/rpc/auth/`
  - `contracts/errors` → `contracts/error-codes` (rename)
- Create stub directories for missing: `rpc/`, `jsonschema/`, `compat/`, `sdk-gen/`
- Update root Cargo.toml
- Verify: `cargo check --workspace`

### Step 4: Handle shared (LOW risk - mostly unused)
- Current: `packages/shared/{errors, utils, config, env, testing, types}`
- Analysis: ALL are unused (zero consumers except shared-errors which is thin wrapper)
- Decision: **Keep for now, mark as TODO for Phase 3**
  - Rationale: Deleting unused code is separate cleanup task
  - No structural harm from keeping stubs
  - Will address in Phase 3 when implementing runtime

### Step 5: Handle features (MEDIUM risk - decision needed)
- Current: `packages/features/{auth, counter, admin, agent, chat, settings, ...}`
- ARCHITECTURE.md: No `features/` directory exists
- Options:
  - A: Delete and merge into services (cleaner, but more work)
  - B: Keep as-is, address in Phase 4 (services integration)
- **Decision: B - Keep for Phase 2, address in Phase 4**
  - Rationale: Features define frontend-facing contracts/types
  - Some are consumed by Tauri adapter (major refactor if removed now)
  - Phase 4 is services integration - better time to decide feature vs service boundary

### Step 6: Handle adapters (MEDIUM risk - structural only)
- Current: `packages/adapters/{telemetry, hosts, storage, auth, cache, chains, protocols}`
- ARCHITECTURE.md: Adapters should nest under capability packages
  - `packages/cache/adapters/`
  - `packages/storage/adapters/`
  - `packages/observability/` (telemetry)
  - `packages/authn/` (auth adapters)
  - `packages/web3/` (chains + protocols)
- **Decision: Defer to Phase 3-4**
  - Rationale: Requires creating new capability packages first
  - Current structure is clean and navigable
  - Moving adapters without moving their consumer packages creates more confusion
  - Will address when implementing `packages/runtime/`, `packages/authn/`, etc.

### Step 7: Handle core/domain and core/workspace-hack
- `packages/core/domain` → Keep as-is for now (Phase 4 services work)
- `packages/core/workspace-hack` → Keep as-is (build optimization)
- `packages/core/state` → Empty stub, delete or keep as placeholder

---

## Phase 2 Scope (What We WILL Do)

1. ✅ Move `packages/core/kernel` → `packages/kernel`
2. ✅ Move `packages/core/platform` → `packages/platform`
3. ✅ Restructure `packages/contracts/` to match ARCHITECTURE.md target structure
4. ✅ Update all Cargo.toml workspace members
5. ✅ Update bun-workspace.yaml if needed
6. ✅ Verify build and validators pass

## Phase 2 Scope (What We WON'T Do - Deferred)

- ❌ Move adapters (deferred to Phase 3-4)
- ❌ Eliminate shared/ (deferred, mostly unused)
- ❌ Eliminate features/ (deferred to Phase 4)
- ❌ Create packages/runtime/ (Phase 3 task)
- ❌ Create packages/authn/, authz/, data/, messaging/, etc. (Phase 3-5 tasks)

---

## Risk Mitigation

1. **Each move is atomic**: Update Cargo.toml, move directory, verify build
2. **No batch moves**: One package at a time, verify after each
3. **Rollback plan**: `git revert` each commit independently
4. **Verification after each step**: `cargo check --workspace` must pass

---

## Execution Order

```
1. Move kernel → packages/kernel
2. cargo check --workspace
3. Move platform → packages/platform
4. cargo check --workspace
5. Restructure contracts
6. cargo check --workspace
7. Update bun-workspace.yaml (if needed)
8. Run all validators
9. git diff --exit-code
```

---

## Success Criteria

- [ ] `packages/kernel/` exists and builds
- [ ] `packages/platform/` exists and builds
- [ ] `packages/contracts/` has target structure (http, events, rpc, jsonschema, error-codes, compat, sdk-gen)
- [ ] `cargo check --workspace` passes with 0 errors
- [ ] All 6 platform validators pass
- [ ] `just gen-platform` succeeds
- [ ] No broken imports
