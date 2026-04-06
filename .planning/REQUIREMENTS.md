# Requirements: Tauri-SvelteKit-Axum Boilerplate

**Defined:** 2026-04-06
**Milestone:** v0.2.1
**Core Value:** Provide a runnable, tested, production-ready engineering base with Google Auth, Counter, Admin Web, Agent conversation, contracts/typegen single-truth-source, and clear architectural boundaries.

## v0.2.1 Requirements

### Release Gate & Evidence

- [ ] **QGATE-01**: Maintainer can merge to protected branches only when Windows desktop E2E required check passes.
- [ ] **QGATE-02**: Maintainer can verify release readiness from Windows and macOS QA/UAT evidence for the same candidate build.
- [ ] **QGATE-03**: Maintainer can view a release quality summary that includes automated test results, UAT sign-offs, and open defects by severity.

### Defect Lifecycle Governance

- [ ] **BUG-01**: Maintainer can triage each bug with a standard severity (P0-P3), workflow state, and owner.
- [ ] **BUG-02**: Maintainer can close P0/P1 bugs only when linked regression verification evidence exists.
- [ ] **BUG-03**: Maintainer can track flaky tests in a dedicated quarantine flow with repair SLA and status visibility.

### Authentication & Session

- [x] **AUTH-02**: Signed-in user can click a visible Google logout action to sign out. — validated in Phase 9
- [x] **AUTH-03**: Signed-in user returns to unauthenticated state after logout with session credentials cleared across desktop and browser paths. — validated in Phase 9

### Multi-tenant Verification

- [ ] **MTEN-01**: Tester can switch between at least two tenants in a repeatable test harness.
- [x] **MTEN-02**: Tester can verify counter values are tenant-scoped, where tenant-1 changes do not alter tenant-2 values.
- [ ] **MTEN-03**: Maintainer can run automated multi-tenant tests in CI and collect artifacts for diagnosis.

### Functional Bug Fixes

- [x] **COUNTER-02**: User can increment and decrement the counter and observe correct value changes in UI and persisted state. — validated in Phase 9
- [x] **AGENT-02**: User can click New Chat and start a new conversation thread. — validated in Phase 9
- [x] **AGENT-03**: User can click New Chat without resetting saved API key, base URL, and model settings. — validated in Phase 9
- [x] **AGENT-04**: User can click a connectivity-test action to validate API key, base URL, and model reachability with actionable result feedback. — validated in Phase 9

## v0.2.x Requirements (Deferred)

### Quality Hardening Enhancements

- **QGATE-04**: Maintainer can enforce merge queue checks for all protected branches with periodic required-check audit.
- **REG-01**: Maintainer can run risk-based selective regression from changed paths with automatic fallback to full-suite on uncertainty.
- **MTEN-04**: Tester can run multi-tenant stress scenarios (parallel tenant mutation + recovery) to detect cross-tenant leakage under load.

## Out of Scope (This Milestone)

| Feature | Reason |
|---------|--------|
| macOS desktop WebDriver parity with Windows | Current ecosystem support is not stable enough for v0.2.1 release gate commitments |
| Unlimited retries to force green CI | Masks regressions and reduces release-signal trust |
| Full replacement of existing test stack (WDIO/Playwright/moon/Just) | Violates brownfield minimal-change strategy and increases migration risk |
| Blocking release on all P2/P3 defects | Would reduce delivery throughput without proportional risk reduction |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| QGATE-01 | Phase 11 | Pending |
| QGATE-02 | Phase 13 | Pending |
| QGATE-03 | Phase 13 | Pending |
| BUG-01 | Phase 12 | Pending |
| BUG-02 | Phase 12 | Pending |
| BUG-03 | Phase 12 | Pending |
| AUTH-02 | Phase 9 | Validated |
| AUTH-03 | Phase 9 | Validated |
| MTEN-01 | Phase 10 | Pending |
| MTEN-02 | Phase 10 | Complete |
| MTEN-03 | Phase 10 | Pending |
| COUNTER-02 | Phase 9 | Validated |
| AGENT-02 | Phase 9 | Validated |
| AGENT-03 | Phase 9 | Validated |
| AGENT-04 | Phase 9 | Validated |

**Coverage:**
- v0.2.1 requirements: 15 total
- Mapped to phases: 15
- Unmapped: 0 ✓

---

*Requirements defined: 2026-04-06*
*Last updated: 2026-04-06 after Phase 9 verification*
