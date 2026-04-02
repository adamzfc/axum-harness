# Create Feature Playbook

**Purpose:** Guide an agent through creating a new feature end-to-end, from contract definition to verification.
**Trigger:** When a new feature module is needed (new API endpoint, new UI page, new Rust feature crate).
**Mode:** High-level steps (3-5 step overview) for everyday feature development.

---

## Trigger

This playbook applies when you need to:
- Add a new API endpoint or route
- Create a new UI page or component
- Add a new Rust feature crate
- Introduce a new capability that spans multiple layers

If you are modifying existing cross-boundary DTOs, use the **update-contracts** playbook instead.

---

## Pre-flight

Before starting, confirm:

- [ ] Read and understand `AGENTS.md` (execution protocol, hard constraints, decision priorities)
- [ ] Read `.agents/rubrics/boundary-compliance.md` (layer import rules)
- [ ] Run `just verify` to confirm current state is clean
- [ ] Identify which layer(s) the feature belongs to:
  - **domain** — port traits, value objects, error types
  - **usecases** — service traits, business logic implementations
  - **adapters/storage** — database/repository implementations
  - **adapters/hosts** — Tauri command handlers
  - **servers** — Axum route handlers
  - **contracts** — DTOs shared across boundaries
  - **frontend** — SvelteKit routes, Svelte components

---

## Execution Steps

Follow this 5-step flow. Skip steps that don't apply to your feature.

### Step 1: Define Contracts

If the feature exposes data across boundaries, define the DTO first:

1. Add new struct(s) in `packages/contracts/api/` (or `auth/`, `events/` as appropriate)
2. Ensure structs derive `#[derive(Serialize, Deserialize, TS, utoipa::ToSchema)]`
3. Do NOT add business logic to contracts — they are pure data shapes

### Step 2: Implement Domain Port Trait

If the feature requires a new capability:

1. Define the trait in `packages/core/domain/`
2. Define any value objects or error types it needs
3. The trait should be framework-agnostic (no tauri, axum, surrealdb imports)

### Step 3: Implement Usecases Service

1. Implement the service trait in `packages/core/usecases/`
2. Define input/output types local to usecases (e.g., `CreateXxxInput`) — do NOT use contracts_api types here
3. Keep business logic in usecases, not in adapters or servers

### Step 4: Implement Adapter

Choose the adapter type:

- **Storage adapter** (`packages/adapters/storage/`): Implement the domain port trait for SurrealDB or libSQL
- **Host adapter** (`packages/adapters/hosts/tauri/`): Create Tauri command handler that delegates to usecases
- **External adapter** (if calling third-party APIs): Wrap the external client behind a domain port trait

### Step 5: Implement Host Entry + Frontend

1. **Server route** (`servers/api/`): Create Axum route handler that maps contracts_api DTOs ↔ usecases types, then delegates
2. **Tauri command** (if desktop feature): Add `#[tauri::command]` in `runtime_tauri/` that delegates to usecases
3. **Frontend** (`frontend/src/routes/`): Create SvelteKit route or component, using types from `frontend/generated/`

---

## Verification

After implementation, run all checks:

```bash
# Generate TypeScript types from contracts
just typegen

# Full quality check (fmt + lint + typecheck + test)
just verify

# Rust unit tests
cargo test

# Boundary compliance: verify no cross-layer import violations
# Review against .agents/rubrics/boundary-compliance.md
```

Additional manual checks:

- [ ] No `unwrap()` in production code (tests are fine)
- [ ] No `console.log` left in frontend code
- [ ] No new compiler warnings
- [ ] Generated files in `frontend/generated/` are committed
- [ ] Function length ≤ 50 lines, file length ≤ 800 lines
- [ ] Nesting depth ≤ 4 levels

---

## Rollback

If something goes wrong:

### Uncommitted changes

```bash
# Stash all changes
git stash

# Or discard all changes
git reset --hard HEAD
```

### Already committed

```bash
# Revert the last commit
git revert HEAD
```

### Clean generated types

```bash
# Remove generated directory and regenerate
rm -rf frontend/generated/
just typegen
```

### If boundary compliance fails

1. Review `.agents/rubrics/boundary-compliance.md` for the violated rule
2. Identify the offending import and move it to the correct layer
3. Re-run `just verify`
