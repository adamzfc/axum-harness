# Update Contracts Playbook

**Purpose:** Guide an agent through modifying cross-boundary DTOs safely, with type generation pipeline and drift prevention.
**Trigger:** When existing contracts (DTOs) need to change — new/modified API request/response structures, Tauri command parameters, event payloads.
**Mode:** Detailed steps with verification at each stage.

---

## Trigger

This playbook applies when you need to:
- Add, remove, or modify fields in API request/response DTOs
- Change Tauri command parameter or return types
- Modify event payload structures
- Update validation rules on shared types

If you are creating a brand new feature from scratch, use the **create-feature** playbook instead.

---

## Pre-flight

Before starting, confirm:

- [ ] Read and understand `AGENTS.md` (execution protocol, hard constraints, decision priorities)
- [ ] Read `.agents/rubrics/boundary-compliance.md` (layer import rules)
- [ ] Run `just verify` to confirm current state is clean
- [ ] Identify the contracts scope:
  - `packages/contracts/api/` — API request/response DTOs
  - `packages/contracts/auth/` — Authentication-related DTOs
  - `packages/contracts/events/` — Event payload DTOs
- [ ] Create a git branch to isolate changes: `git checkout -b update-contracts-<feature>`

---

## Execution Steps

### Step 1: Modify DTO Structs in Contracts Crate

1. Open the relevant contract file in `packages/contracts/`
2. Add, remove, or modify fields on the struct
3. Ensure backward compatibility where possible:
   - New fields should have `#[serde(default)]` if consumers may not send them
   - Renaming fields requires `#[serde(rename = "old_name")]` during transition
   - Removing fields is a breaking change — coordinate with all consumers

### Step 2: Ensure Correct Derive Attributes

Every contract struct must have:

```rust
#[derive(Serialize, Deserialize, Clone, Debug, TS, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MyDto {
    // fields
}
```

Required derives:
- `Serialize`, `Deserialize` — JSON serialization
- `Clone`, `Debug` — Standard Rust ergonomics
- `TS` (ts-rs) — TypeScript type generation
- `utoipa::ToSchema` — OpenAPI documentation

### Step 3: Run Type Generation

```bash
just typegen
```

This generates TypeScript types into `frontend/generated/`. Verify:

- [ ] Command exits with code 0
- [ ] New types appear in `frontend/generated/`
- [ ] Modified types reflect the struct changes
- [ ] No unexpected deletions in generated files

### Step 4: Update Rust Consumers

Update all Rust code that consumes the modified contracts:

1. **Server routes** (`servers/api/`):
   - Update route handler to use new/modified DTO fields
   - Update request/response mapping logic

2. **Tauri commands** (`runtime_tauri/`):
   - Update command handler signatures if return types changed
   - Update argument destructuring if parameters changed

3. **Usecases** (`packages/core/usecases/`):
   - Update input/output types if they map to the changed contracts
   - Remember: usecases defines its OWN types, NOT contracts_api types — update the mapping layer

### Step 5: Update TypeScript Consumers

Update all frontend code that uses the generated types:

1. **SvelteKit routes** (`frontend/src/routes/`):
   - Update `+page.server.ts` load functions to use new types
   - Update form actions if request/response shapes changed

2. **Svelte components** (`frontend/src/lib/`):
   - Update component props that reference changed types
   - Update API client calls with new request bodies

3. **Type imports**:
   - Import from `frontend/generated/` — do NOT manually type what should be generated

### Step 6: Verify No New Drift

```bash
just typegen
```

Run typegen again to confirm no new drift was introduced by your consumer updates. If new files are generated, it means your Rust changes created additional types — review them for correctness.

### Step 7: Full Verification

```bash
just verify
```

This runs the full quality pipeline: fmt, lint, typecheck, and tests.

---

## Verification

Run all checks explicitly:

```bash
# Type generation — must succeed with no uncommitted diff
just typegen

# Full quality check
just verify

# Rust tests
cargo test
```

Manual verification:

- [ ] `frontend/generated/` files are `git add`'d (not drifting)
- [ ] No contracts import from domain, usecases, adapters, or servers
- [ ] All derive attributes present (`Serialize`, `Deserialize`, `TS`, `ToSchema`)
- [ ] TypeScript consumers compile without errors
- [ ] No `unwrap()` in production code
- [ ] No `console.log` in frontend code

### Boundary Compliance Check

Review against `.agents/rubrics/boundary-compliance.md`:

- [ ] contracts_* does NOT import from domain, usecases, adapters, servers
- [ ] contracts_* only imports: serde, ts-rs, utoipa, validator
- [ ] No business logic leaked into contracts

---

## Rollback

### Uncommitted changes

```bash
# Discard contracts changes
git checkout -- packages/contracts/

# Clean generated types
rm -rf frontend/generated/

# Regenerate from current contracts
just typegen
```

### Already committed

```bash
# Revert the last commit
git revert HEAD

# Regenerate types to match reverted contracts
rm -rf frontend/generated/
just typegen
```

### If typegen fails

1. Check contracts crate compilation: `cargo check -p contracts_api`
2. Fix any compile errors in the contract structs
3. Ensure all derive macros are present and correct
4. Re-run `just typegen`

---

## Drift Prevention

### Why Contracts Are the Single Source of Truth

The contracts crates (`packages/contracts/api`, `auth`, `events`) define the **only** authoritative shapes for data that crosses boundaries. Both Rust consumers and TypeScript consumers derive their types from these structs via:

- **Rust side**: Direct `use contracts_api::MyDto` imports
- **TypeScript side**: Generated types in `frontend/generated/` via `ts-rs`

This means:
- If a field exists in contracts, it exists everywhere
- If a field is missing from contracts, it should not exist anywhere
- Manual TypeScript types that duplicate contract shapes are a code smell

### How CI Drift Check Works

The CI pipeline includes a drift check that:

1. Runs `just typegen` to regenerate types
2. Runs `git diff --exit-code` on `frontend/generated/`
3. If diff is non-empty, the check fails — meaning generated types don't match committed types

This catches two problems:
- **Forgotten typegen**: You changed contracts but didn't run `just typegen`
- **Manual edits**: Someone manually edited `frontend/generated/` files (they should never do this)

### Common Drift Scenarios and Fixes

| Scenario | Symptom | Fix |
|----------|---------|-----|
| Changed contracts, forgot typegen | TypeScript compile errors | Run `just typegen`, commit generated files |
| Manually edited generated types | CI drift check fails | Revert manual edits, run `just typegen` |
| Added struct without `TS` derive | Type not generated | Add `TS` derive, run `just typegen` |
| Renamed field without `serde(rename)` | Runtime deserialization error | Add rename attribute or update all consumers |
| Removed field without updating consumers | Compile errors in servers/frontend | Update all consumers first, then remove field |
