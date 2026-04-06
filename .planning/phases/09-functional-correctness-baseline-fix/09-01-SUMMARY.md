---
phase: 09-functional-correctness-baseline-fix
plan: 01
subsystem: auth
tags: [tauri, sveltekit, auth, logout, diagnostics, vitest]

requires:
  - phase: 08
    provides: dual-path IPC baseline and authenticated app shell
provides:
  - Settings page logout action wired to explicit Tauri logout command
  - Dual-path sign-out that always clears local auth credentials
  - Tri-state connection diagnostics for API key/Base URL/Model with guidance
  - Regression tests for logout fallback and non-destructive settings diagnostics
affects: [phase-09, auth, settings, agent]

tech-stack:
  added: []
  patterns:
    - Remote logout best-effort + local credential clear hard guarantee
    - Per-dimension connection diagnostics with actionable recommendations

key-files:
  created:
    - apps/client/web/app/tests/component/settings-connection.test.ts
  modified:
    - packages/adapters/hosts/tauri/src/commands/auth.rs
    - apps/client/native/src-tauri/src/lib.rs
    - apps/client/web/app/src/lib/ipc/auth.ts
    - apps/client/web/app/src/lib/stores/auth.svelte.ts
    - apps/client/web/app/src/routes/(app)/settings/+page.svelte
    - apps/client/web/app/tests/component/auth.test.ts

key-decisions:
  - "Logout keeps deterministic local cleanup even when remote invalidation fails."
  - "Settings diagnostics are split into API key/Base URL/Model rows with independent pass/fail + next step."

patterns-established:
  - "Auth teardown pattern: await logout() in try/catch, await clearAuthStore() in finally-like path."
  - "Connection test pattern: validate inputs, probe /models with timeout, render per-dimension status without mutating form values."

requirements-completed: [AUTH-02, AUTH-03, AGENT-04]

duration: 29min
completed: 2026-04-06
---

# Phase 09 Plan 01: Settings logout + connection diagnostics Summary

**Settings now exposes deterministic logout and three-way connection diagnostics so users can safely sign out and troubleshoot API configuration without losing inputs.**

## Performance

- **Duration:** 29 min
- **Started:** 2026-04-06T03:54:00Z
- **Completed:** 2026-04-06T04:23:00Z
- **Tasks:** 2
- **Files modified:** 7 (plus 1 new test file)

## Accomplishments
- Added explicit `logout` Tauri command and registered it in native invoke handler.
- Implemented frontend logout IPC and upgraded auth store `signOut()` to remote-first + local-guaranteed cleanup with unauthenticated redirect fallback.
- Added Settings `Logout` + `Test Connection` actions and tri-state diagnostic cards for `API key`, `Base URL`, `Model` with actionable next-step text.
- Added/updated component tests validating logout fallback behavior and non-destructive connection test behavior.

## Task Commits

1. **Task 1: Add explicit logout command and frontend dual-path sign-out** - `46ad92b` (feat)
2. **Task 2: Implement Settings logout + Test Connection tri-state diagnostics** - `c7eda1e` (feat)

## Files Created/Modified
- `packages/adapters/hosts/tauri/src/commands/auth.rs` - added `#[tauri::command] logout` entrypoint.
- `apps/client/native/src-tauri/src/lib.rs` - registered `auth::logout` in invoke handler.
- `apps/client/web/app/src/lib/ipc/auth.ts` - added `logout(): Promise<void>` IPC wrapper.
- `apps/client/web/app/src/lib/stores/auth.svelte.ts` - implemented remote-first sign-out with guaranteed local clear and redirect fallback.
- `apps/client/web/app/src/routes/(app)/settings/+page.svelte` - added Logout/Test Connection UI and per-dimension diagnostic rendering.
- `apps/client/web/app/tests/component/auth.test.ts` - added ordering/fallback assertions for logout failure path.
- `apps/client/web/app/tests/component/settings-connection.test.ts` - added diagnostics coverage for visible actions, tri-state results, and value persistence on failure.

## Decisions Made
- Chose explicit remote logout command invocation from UI layer to satisfy trust-boundary requirement while preserving local logout guarantee.
- Chose per-row diagnostics over a single aggregate state to match D-13 and improve user troubleshooting precision.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Reverted `$app/paths` resolve usage in auth store**
- **Found during:** Task 2 verification (auth tests)
- **Issue:** test environment failed to resolve `$app/paths`, breaking auth test execution.
- **Fix:** reverted to direct `goto('/login' | '/')` calls.
- **Files modified:** `apps/client/web/app/src/lib/stores/auth.svelte.ts`
- **Verification:** `bun run --cwd apps/client/web/app test:unit -- tests/component/auth.test.ts` passed.
- **Committed in:** `c7eda1e` (included in task commit stream)

**2. [Rule 1 - Bug] Replaced `onMount` loader with immediate async init in settings page**
- **Found during:** Task 2 verification (settings component tests)
- **Issue:** component tests raised `effect_orphan` around lifecycle initialization.
- **Fix:** switched to module-time `void loadSettings()` initialization path.
- **Files modified:** `apps/client/web/app/src/routes/(app)/settings/+page.svelte`
- **Verification:** `bun run --cwd apps/client/web/app test:unit -- tests/component/settings-connection.test.ts` passed.
- **Committed in:** `c7eda1e`

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both fixes were verification blockers and remained within planned scope.

## Issues Encountered
- Selector collisions in settings test assertions (`Base URL` label vs diagnostic row) required test-targeted `data-testid` checks.

## Known Stubs
None.

## Threat Flags
None.

## Next Phase Readiness
- AUTH-02/AUTH-03/AGENT-04 behaviors now have executable component-level coverage and deterministic logout semantics.
- Plan 09-02 can proceed on top of this baseline.

## Self-Check: PASSED
- Summary file exists and both task commits are present in git history.
