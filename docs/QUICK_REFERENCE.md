# Quick Reference Card for Refactoring

> Print this or keep it open in a tab for fast reference  
> Last updated: 2026-04-12

---

## The 7 Golden Rules (from ARCHITECTURE.md §8)

1. **先改 platform model，再改 infra** - Platform model first, then infra
2. **先改 contracts，再改 server handler** - Contracts first, then handlers
3. **services 是库，不是进程** - Services are libraries, not processes
4. **workers 是一等公民，不是附属脚本** - Workers are first-class, not scripts
5. **vendor 只能进 adapters** - Vendor code only in adapters
6. **generated 目录禁止手改** - No hand-editing generated dirs
7. **拓扑变化靠 topology model，不靠重构业务** - Topology changes via model, not refactoring

---

## Dependency Rules (from ARCHITECTURE.md §2.2)

```
apps/*          -> packages/sdk, packages/ui, packages/authn
servers/*       -> services/*, packages/*
workers/*       -> services/*, packages/*
services/*      -> packages/kernel, packages/platform, packages/contracts
packages/*      -> 低层可互依，不得反向依赖 servers/apps/workers/services
platform/*      -> 不依赖业务实现
```

### ❌ NEVER
- `apps/*` → `services/*`
- `services/*` → other `services/*`
- `services/*` → `packages/*/adapters/`
- Business logic in `servers/*/handlers/`
- Framework imports in `services/*/domain/`

---

## Service Structure (minimum complete)

```
services/<name>/
├── Cargo.toml
├── src/
│   ├── domain/          # Entities, value objects, invariants
│   ├── application/     # Use cases, commands, queries
│   ├── policies/        # Business policies
│   ├── ports/           # Interface abstractions
│   ├── infrastructure/  # Adapter implementations
│   ├── events/          # Event definitions
│   ├── contracts/       # DTOs
│   └── lib.rs
├── tests/
├── migrations/
└── README.md
```

---

## Worker Structure (minimum)

```
workers/<name>/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── jobs_or_consumers/
│   └── checkpoint_or_dedupe/
└── README.md
```

---

## Server Structure (minimum)

```
servers/<name>/
├── Cargo.toml
├── openapi.yaml
├── src/
│   ├── handlers/        # Thin, protocol adaptation only
│   ├── middleware/      # Auth, telemetry, CORS, etc.
│   ├── routes/          # Route registration
│   └── main.rs
└── README.md
```

---

## Essential Commands

### Build & Test
```bash
just build              # Build entire workspace
just test               # Run all tests
just verify             # Full verification suite
just doctor             # Platform health check
just dev                # Start dev servers
```

### Validation
```bash
just validate-platform  # Validate platform models
just validate-deps      # Check dependency graph
just validate-contracts # Detect contract drift
just boundary-check     # Architecture boundary check
just contracts-check    # Contract consistency check
```

### Generation
```bash
gen-platform            # Generate platform catalog
gen-sdk                 # Generate SDK from contracts
gen-contracts           # Generate contracts
```

### Inspection
```bash
cargo tree -p <crate>   # Show dependencies
rg "pattern" --type rust  # Search Rust code
git diff --stat         # What changed
git log --oneline -10   # Recent history
```

---

## Golden Examples (copy these patterns)

| What | Where | Why |
|------|-------|-----|
| Complete service | `services/counter-service/` | Has all layers, tests, migrations |
| HTTP route | `servers/api/src/routes/counter.rs` | Clean handler pattern |
| Frontend page | `apps/web/src/routes/(app)/counter/+page.svelte` | SvelteKit pattern |
| Tauri command | `packages/adapters/hosts/tauri/src/commands/counter.rs` | Tauri v2 pattern |
| Feature trait | `packages/features/counter/src/lib.rs` | Trait-only, no impl |
| Contract | `packages/contracts/api/src/counter.rs` | DTO definitions |

---

## Current Service Status

| Service | Status | Missing | Notes |
|---------|--------|---------|-------|
| counter | ✅ Complete | Nothing | Golden example |
| settings | ✅ Complete | Nothing | Golden example #2 |
| user | ⚠️ Partial | HTTP routes stub | Remove axum dep (Phase 1) |
| tenant | ✅ Complete | Nothing | Migrated from core |
| agent | ✅ Complete | Nothing | Migrated from core |
| admin | ⚠️ Partial | ports/infrastructure | Remove cross-service deps (Phase 1) |
| event-bus | ⚠️ Partial | NATS consumer | Move to workers/ (Phase 3) |
| chat | ❌ Stub | Everything | Implement from scratch (Phase 6) |

---

## Phase Quick Lookup

| Phase | Mission | Blocks | Parallel With |
|-------|---------|--------|---------------|
| 1 | Fix dep violations | Phase 2, 6 | Nothing (must be first) |
| 2 | Create platform/ | Phase 3, 4, 5, 7 | Nothing (must be after 1) |
| 3 | Create workers/ | Phase 8 | Phase 4, 5, 6 |
| 4 | Create verification/ | Phase 8 | Phase 3, 5, 6 |
| 5 | Restructure servers/ | Phase 8 | Phase 3, 4, 6 |
| 6 | Complete services | Phase 8 | Phase 3, 4, 5 |
| 7 | Add commands/CI | Phase 8 | Phase 3, 4, 5, 6 |
| 8 | Final verification | Done | Nothing (must be last) |

---

## Common Violations to Check

```bash
# Cross-service dependencies
for svc in services/*/; do
  cargo tree -p $(basename $svc) | grep "services/"
done

# Framework imports in domain layer
rg "axum|tauri|hyper|reqwest" services/*/src/domain/

# Direct adapter imports in services
rg "packages/adapters/(turso|surrealdb|moka)" services/*/src/

# Business logic in handlers
rg "if.*business|match.*domain" servers/*/src/handlers/
```

---

## File Locations Cheat Sheet

| Need | Look In | Example |
|------|---------|---------|
| Business entities | `services/<name>/src/domain/` | Counter entity |
| Use cases | `services/<name>/src/application/` | Increment counter |
| Port traits | `services/<name>/src/ports/` | CounterRepository trait |
| Implementations | `services/<name>/src/infrastructure/` | Turso adapter |
| HTTP DTOs | `packages/contracts/api/` | CounterResponse |
| Feature traits | `packages/features/<domain>/` | CounterService trait |
| HTTP routes | `servers/api/src/routes/` | /api/counter/* |
| BFF handlers | `servers/bff/*/handlers/` | Web BFF handlers |
| Frontend pages | `apps/web/src/routes/` | SvelteKit routes |
| Tauri commands | `packages/adapters/hosts/tauri/src/commands/` | Desktop commands |

---

## Debugging Quick Guide

### Service won't compile
```bash
cargo check -p <service>  # See errors
cargo tree -p <service>   # Check deps
rg "use .*from" services/<name>/src/  # Check imports
```

### Dependency violation
```bash
cargo tree -p <service> | grep "services/"  # Find violation
cat services/<service>/Cargo.toml  # Check deps
# Fix: Remove dep, extract to contract/port
```

### Test failing
```bash
cargo test -p <service> -- --nocapture  # See output
cargo test -p <service> <test_name>  # Run specific test
# Check if it's integration test needing DB
```

### Generated file drift
```bash
just gen-contracts  # Regenerate
git diff packages/contracts/generated/  # See drift
# If expected: commit new generation
# If unexpected: fix generator or source
```

---

## Emergency Contacts

| Problem | Solution |
|---------|----------|
| Broke the build | `git revert <commit>` |
| Lost context | Re-read ARCHITECTURE.md + this file |
| Don't understand | Look at counter-service as example |
| Need to pause | `git stash` and document state |
| Uncertain | Ask, don't guess |

---

## Daily Checklist

### Starting work
- [ ] Read `docs/PHASE_HANDOFF.md` (current state)
- [ ] Check `git status && git log --oneline -5`
- [ ] Identify your phase from status board
- [ ] Read phase brief and full plan
- [ ] Run `just build` (ensure clean state)

### During work
- [ ] Follow golden example patterns
- [ ] Test after each logical unit
- [ ] Commit frequently with clear messages
- [ ] Update task list as you go

### Finishing work
- [ ] Run full verification: `just verify`
- [ ] Write phase completion report
- [ ] Update `docs/PHASE_HANDOFF.md` status board
- [ ] Ensure git commit is clean and atomic
- [ ] Document any known issues for next agent

---

**Remember**: 
- Constitution: `docs/ARCHITECTURE.md`
- Plan: `docs/REFACTORING_PLAN.md`
- Map: `docs/PHASE_HANDOFF.md`
- Pattern: `services/counter-service/`
- Rule: Read more, guess less. Test early, test often.
