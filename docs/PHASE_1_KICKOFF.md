# Phase 1 Kickoff: Fix Dependency Violations

> **Status**: READY TO START  
> **Priority**: CRITICAL (blocks all other phases)  
> **Risk**: HIGH (may break existing code)  
> **Estimated effort**: 1-2 days  
> **Handoff**: Complete when all services pass `cargo test` with zero violations

---

## Mission

Fix architecture dependency violations in existing services to comply with `docs/ARCHITECTURE.md` §2.2 rules.

### Two Main Violations

1. **admin-service** depends on `tenant-service` and `counter-service` ❌
2. **user-service** depends on `axum` (HTTP framework) ❌

---

## Pre-Flight Checklist

Before starting, verify:

```bash
# Current state is clean
git status  # No uncommitted changes (or stashed)
just build  # Everything compiles currently
just test   # All tests pass currently
```

If anything fails, **fix first** or **stash and document**.

---

## Task 1.1: Fix admin-service Cross-Service Dependencies

### Current State

**File**: `services/admin-service/Cargo.toml`

Look for lines like:
```toml
[dependencies]
tenant-service = { path = "../tenant-service" }
counter-service = { path = "../counter-service" }
```

### What admin-service is doing wrong

Admin service is a **composition layer** - it aggregates data from multiple services to build dashboard views. This composition should happen in **servers/bff/admin-bff** or **servers/api**, not in the service layer.

### Solution Strategy

**Step 1**: Understand what admin-service uses from tenant-service and counter-service

```bash
# Read admin-service source
rg "tenant_service|counter_service" services/admin-service/src/
# Or read the files directly
find services/admin-service/src/ -name "*.rs" -exec cat {} \;
```

**Step 2**: Extract shared contracts

If admin-service is using types from tenant/counter services, those types should be in `packages/contracts/admin/` instead.

Create: `packages/contracts/admin/src/dashboard.rs`
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminDashboardStats {
    pub tenant_count: u64,
    pub active_counters: u64,
    // ... other aggregated fields
}
```

**Step 3**: Define composition interfaces in features

Create: `packages/features/admin/src/lib.rs`
```rust
use async_trait::async_trait;

#[async_trait]
pub trait AdminDashboardPort {
    async fn get_dashboard_stats(&self) -> Result<AdminDashboardStats, AdminError>;
}
```

**Step 4**: Move composition logic to server layer

The actual aggregation of multiple services should be in `servers/api/src/routes/admin.rs` or `servers/bff/admin-bff/src/handlers/dashboard.rs`.

**Step 5**: Remove cross-service dependencies

Edit `services/admin-service/Cargo.toml`:
```diff
[dependencies]
- tenant-service = { path = "../tenant-service" }
- counter-service = { path = "../counter-service" }
```

**Step 6**: Update admin-service code

Replace direct service calls with:
- Contract types from `packages/contracts/admin/`
- Port traits from `packages/features/admin/`
- Or move the composition to server layer entirely

### Verification

```bash
# Should compile
cargo check -p admin-service

# Should pass tests
cargo test -p admin-service

# Should have NO cross-service deps
cargo tree -p admin-service | grep -E "tenant|counter"
# Expected: (empty output)
```

---

## Task 1.2: Fix user-service Axum Dependency

### Current State

**File**: `services/user-service/Cargo.toml`

Look for:
```toml
[dependencies]
axum = "0.7"
```

### What user-service is doing wrong

Services should not depend on HTTP frameworks. The `axum` dependency suggests user-service has HTTP-specific types or logic in the business layer.

### Solution Strategy

**Step 1**: Find axum usage in user-service

```bash
rg "axum" services/user-service/src/
# Note what's being used: extractors? responses? routing?
```

**Step 2**: Move HTTP types to server layer

If user-service defines axum-specific types (like `Json<UserResponse>`), move them to:
- `servers/api/src/routes/user.rs` for route-specific types
- `packages/contracts/api/src/user.rs` for shared DTOs

**Step 3**: Remove axum dependency

Edit `services/user-service/Cargo.toml`:
```diff
[dependencies]
- axum = "0.7"
```

**Step 4**: Update user-service code

Replace any axum-specific code with pure Rust types.

Example - BEFORE (wrong):
```rust
// In services/user-service/src/application/user_service.rs
use axum::Json;

pub async fn get_user(id: UserId) -> Json<UserResponse> {
    let user = self.repo.find(id).await;
    Json(UserResponse::from(user))
}
```

Example - AFTER (correct):
```rust
// In services/user-service/src/application/user_service.rs
pub async fn get_user(&self, id: UserId) -> Result<User, AppError> {
    self.repo.find(id).await
}

// In servers/api/src/routes/user.rs (server layer)
async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<UserId>,
) -> Json<UserResponse> {
    let user = state.user_service.get_user(id).await.unwrap();
    Json(UserResponse::from(user))
}
```

### Verification

```bash
# Should compile
cargo check -p user-service

# Should pass tests
cargo test -p user-service

# Should have NO axum imports
rg "axum" services/user-service/src/
# Expected: (empty output)

# Should have NO axum in deps
cargo tree -p user-service | grep axum
# Expected: (empty output)
```

---

## Task 1.3: Audit All Service Dependencies

### Run Full Audit

```bash
#!/bin/bash
# Save as /tmp/audit_deps.sh and run

echo "=== Cross-service dependency check ==="
for svc in services/*/; do
  svc_name=$(basename "$svc" -service)
  echo "--- $svc_name-service ---"
  cargo tree -p "${svc_name}-service" 2>/dev/null | grep "services/" || echo "✅ OK"
done

echo ""
echo "=== Framework imports in domain layer ==="
rg "axum|tauri|hyper|reqwest" services/*/src/domain/ || echo "✅ OK"

echo ""
echo "=== Direct adapter imports in services ==="
rg "packages/adapters/(turso|surrealdb|moka)" services/*/src/ || echo "✅ OK"
```

### Document Any Remaining Violations

If you find other violations not in the plan, document them:

Create: `docs/refactoring/known-violations.md`
```markdown
# Known Dependency Violations

## Critical (must fix before Phase 2)
- [ ] admin-service → tenant-service, counter-service (Task 1.1)
- [ ] user-service → axum (Task 1.2)

## Non-critical (track for later)
- [ ] <service-a> → <service-b> (describe why, plan to fix)
```

---

## Testing Strategy

### Unit Tests (should already exist)

```bash
# Each service should have independent tests
cargo test -p admin-service --lib
cargo test -p user-service --lib
```

### Integration Tests (may need DB)

```bash
# Run integration tests if they exist
cargo test -p admin-service --test '*'
cargo test -p user-service --test '*'
```

### Boundary Check

```bash
# After fixes, run boundary checker
just boundary-check
# Should pass with zero violations
```

---

## Common Pitfalls

### ❌ Don't Do This

1. **Don't just comment out the dependencies** - Code won't compile
2. **Don't move business logic to servers** - Only move composition/aggregation
3. **Don't break existing functionality** - Tests must still pass
4. **Don't introduce new abstractions prematurely** - Use existing contracts/features

### ✅ Do This Instead

1. **Understand first** - Read the code to see what's actually being used
2. **Extract minimal contracts** - Only pull out what's needed
3. **Move composition to server layer** - Admin dashboard is a VIEW concern
4. **Test incrementally** - After each service fix, verify it compiles and tests pass

---

## Rollback Plan

If something breaks badly:

```bash
# See what changed
git diff

# Revert this phase's commits
git revert HEAD  # If one commit
git revert <sha1> <sha2>  # If multiple commits

# Or stash work and start fresh
git stash save "Phase 1 WIP - need to rethink approach"
```

---

## Completion Checklist

- [ ] admin-service has NO cross-service dependencies
- [ ] user-service has NO axum dependency
- [ ] All services pass `cargo test`
- [ ] `cargo tree` audit shows zero violations
- [ ] `just boundary-check` passes
- [ ] `just test` passes (full workspace)
- [ ] Created `docs/refactoring/phase-1-completion.md`
- [ ] Updated `docs/PHASE_HANDOFF.md` status board
- [ ] Committed with clear message

---

## Git Commit Message Template

```text
fix(deps): remove architecture dependency violations

- Remove admin-service dependency on tenant-service and counter-service
  (violates service isolation rule, composition moved to server layer)
- Remove user-service dependency on axum
  (violates layering, HTTP types moved to servers/api)
- Extract shared contracts to packages/contracts/admin/
- Define composition interfaces in packages/features/admin/

Verification:
- cargo test -p admin-service ✅
- cargo test -p user-service ✅
- cargo tree audit shows zero cross-service deps ✅
- just boundary-check ✅
```

---

## Next Phase Handoff

When complete, the next agent (Phase 2: platform/) will need:

1. **This completion report** - What was changed and why
2. **Clean dependency tree** - Verified with `cargo tree`
3. **All tests passing** - Verified with `just test`
4. **Updated status board** - In `docs/PHASE_HANDOFF.md`

Handoff message template:

```markdown
# Phase 1 → Phase 2 Handoff

**Phase 1 status**: COMPLETE ✅
**Git commit**: `<sha>`
**Completion report**: `docs/refactoring/phase-1-completion.md`

## What was delivered
- ✅ admin-service: No cross-service dependencies
- ✅ user-service: No axum dependency
- ✅ All services pass independent tests
- ✅ Dependency audit clean

## Known issues
- None (or list if any)

## Phase 2 readiness
Phase 2 (platform/) can now start. All dependency violations fixed.
Services are clean and follow architecture rules.
```

---

## Questions?

- **Unclear what admin-service is doing?** Read `services/admin-service/src/application/*.rs`
- **Don't understand ports vs contracts?** See `services/counter-service/` as example
- **Need help with axum types?** Check `servers/api/src/routes/` for server-layer patterns
- **Architecture rules confusing?** Re-read `docs/ARCHITECTURE.md` §2.2 and §3.6

---

**Remember**: 
- Read code BEFORE changing it
- Test AFTER each logical unit
- Minimal changes, maximum evidence
- This is Phase 1 - foundation for everything else

**Good luck!** 🚀
