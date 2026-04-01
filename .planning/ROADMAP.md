# ROADMAP: Tauri-SvelteKit-Axum Boilerplate

**Generated:** 2026-04-01
**Milestone:** v0.1.1
**Granularity:** fine
**Total v0.1.1 Requirements:** 9
**Active Plan Range:** Phase 11-15

## Phases

- [ ] **Phase 11: Security Baseline Closure** - Close JWT validation, secret fail-fast, and path portability gaps for production readiness
- [ ] **Phase 12: Contract-First Type Sync** - Make `contracts_api` the single contract source and enforce Rust→TS drift prevention
- [ ] **Phase 13: Runtime Boundary Convergence** - Move new orchestration ownership into `runtime_tauri` and keep native host bootstrap-only
- [ ] **Phase 14: Workflow Guardrails Unification** - Provide unified Moon/Just entrypoints for fullstack dev, typegen, and verify
- [ ] **Phase 15: Decision Ledger & Forward Map** - Record all strategy decisions with rationale and explicit future-phase triggers

## Phase Details

### Phase 11: Security Baseline Closure
**Goal**: Users and operators can trust that auth-critical traffic, environment secrets, and runtime paths behave safely across environments.
**Depends on**: Existing baseline through Phase 10
**Requirements**: SEC-01, SEC-02, SEC-03
**Success Criteria** (what must be TRUE):
  1. Forged/invalid JWTs are rejected on tenant/auth-critical endpoints with unauthorized responses.
  2. In non-dev environments, startup fails immediately with a clear error when required secrets are missing.
  3. Project can run from a different machine/path without editing hardcoded absolute paths.
**Plans**: TBD

### Phase 12: Contract-First Type Sync
**Goal**: Rust and TypeScript share one contract truth source and cannot silently drift.
**Depends on**: Phase 11
**Requirements**: CONTRACT-01, CONTRACT-02
**Success Criteria** (what must be TRUE):
  1. Shared DTO/contracts are defined in `contracts_api` and consumed by runtime code instead of duplicate handwritten schemas.
  2. Running `typegen` produces TS types directly from Rust contracts and updates generated artifacts deterministically.
  3. `verify`/CI fails when generated contract outputs are stale or drifted, and passes once regenerated.
**Plans**: TBD

### Phase 13: Runtime Boundary Convergence
**Goal**: Runtime orchestration responsibilities are clearly owned by `runtime_tauri`, with native host remaining a thin shell.
**Depends on**: Phase 12
**Requirements**: RUNTIME-01
**Success Criteria** (what must be TRUE):
  1. Newly introduced orchestration logic is implemented under `runtime_tauri`, not in `apps/client/native/src-tauri` host bootstrap code.
  2. Native host entry remains focused on builder/bootstrap responsibilities and still launches successfully.
  3. Existing core app startup and command flow continue to work after boundary convergence.
**Plans**: TBD

### Phase 14: Workflow Guardrails Unification
**Goal**: Developers have one consistent command surface to run, sync, and verify the stack.
**Depends on**: Phase 13
**Requirements**: WF-01
**Success Criteria** (what must be TRUE):
  1. Developers can start the full stack through a single `fullstack:dev` entrypoint.
  2. Developers can run a single `typegen` entrypoint that regenerates contract-derived TS types.
  3. Developers can run a single `verify` entrypoint that enforces quality gates (including type drift checks).
**Plans**: TBD

### Phase 15: Decision Ledger & Forward Map
**Goal**: All milestone strategy decisions are auditable now and actionable in future milestones.
**Depends on**: Phase 14
**Requirements**: DECISION-01, DECISION-02
**Success Criteria** (what must be TRUE):
  1. Every strategy item from the v0.1.1 discussion is recorded with status (`implement-now` / `defer` / `reject`) and rationale.
  2. Every deferred/rejected item includes a target future phase mapping and explicit promotion trigger.
  3. Team can inspect one ledger source to understand what is implemented now versus postponed and why.
**Plans**: TBD

## Progress Table

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 11. Security Baseline Closure | 0/TBD | Not started | - |
| 12. Contract-First Type Sync | 0/TBD | Not started | - |
| 13. Runtime Boundary Convergence | 0/TBD | Not started | - |
| 14. Workflow Guardrails Unification | 0/TBD | Not started | - |
| 15. Decision Ledger & Forward Map | 0/TBD | Not started | - |

## Coverage Map (v0.1.1)

| Requirement | Phase | Status |
|-------------|-------|--------|
| SEC-01 | Phase 11 | Pending |
| SEC-02 | Phase 11 | Pending |
| SEC-03 | Phase 11 | Pending |
| CONTRACT-01 | Phase 12 | Pending |
| CONTRACT-02 | Phase 12 | Pending |
| RUNTIME-01 | Phase 13 | Pending |
| WF-01 | Phase 14 | Pending |
| DECISION-01 | Phase 15 | Pending |
| DECISION-02 | Phase 15 | Pending |

**Coverage: 9/9 v0.1.1 requirements mapped ✓**

## Prior Milestone History (Context Preserved)

Previous roadmap phases remain historical context and are retained for continuity:

- Phase 1: Package Foundation
- Phase 2: UI Styling Infrastructure
- Phase 3: Application Pages
- Phase 4: Backend Dependencies & Build Optimization
- Phase 5: Database & Infrastructure
- Phase 6: Google OAuth Authentication
- Phase 7: Multi-Tenant Data Isolation
- Phase 8: Desktop Native Features
- Phase 9: Cross-Platform Build Pipeline
- Phase 10: Test Suite

---

*Roadmap updated: 2026-04-01*
*Ready for phase planning: `/gsd-plan-phase 11`*
