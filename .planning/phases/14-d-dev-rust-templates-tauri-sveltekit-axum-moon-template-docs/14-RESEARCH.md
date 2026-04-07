# Phase 14 Research — Tauri Playwright Migration (Phase 1)

**Date:** 2026-04-07  
**Scope:** `docs/TAURI_PLAYWRIGHT_MIGRATION_CONTEXT.md` + `14-CONTEXT.md` locked decisions D-01..D-10  
**Outcome:** Ready for executable planning

## Summary

Phase 14 should implement a **reversible Phase 1 migration** from `WDIO + tauri-driver` to a new independent `tauri-playwright` desktop suite, while keeping existing WDIO and web Playwright pipelines running in parallel.

Research confirms the following implementation direction is technically valid and aligns with user-locked decisions:

1. `tauri-plugin-playwright` must be **feature-gated** and only enabled in non-release automation contexts (D-09).
2. New suite should be in a **separate root** (e.g. `e2e-desktop-playwright/`) to avoid coupling with existing `e2e-tests/` WDIO and web Playwright config (D-03).
3. Suite should start with migrated **smoke/login/counter** only (D-01, D-02), while WDIO remains intact as rollback path (D-06).
4. CI should add **macOS observer job** for tauri-playwright and keep existing required lanes unchanged for now (D-05, D-06).
5. Acceptance must enforce **full repository E2E green** and artifact diagnostics across all three tracks (WDIO + web Playwright + tauri-playwright) (D-07, D-08).

## Repository Baseline Findings

- Existing desktop WDIO:
  - `e2e-tests/wdio.conf.mjs`
  - `e2e-tests/scripts/run-desktop-e2e.mjs`
  - `e2e-tests/specs/{smoke,login,counter,admin,agent,tenant-isolation}.e2e.mjs`
- Existing web Playwright:
  - `apps/client/web/app/playwright.config.ts`
  - `apps/client/web/app/tests/e2e/*.test.ts`
- Existing CI (`.github/workflows/e2e-tests.yml`) already has:
  - `desktop-e2e` (WDIO, ubuntu/windows)
  - `web-e2e` (Playwright, ubuntu/windows/macos)
  - evidence upload with job-scoped naming and retention

## External Documentation Findings

### Tauri / plugin gating

- Tauri docs support conditional plugin initialization via `#[cfg(debug_assertions)]` style guard.
- This pattern extends to custom feature-gated plugins in `Cargo.toml` + conditional `.plugin(...)` registration in `src/lib.rs`.
- Capability permissions are required in `src-tauri/capabilities/default.json` for plugin command surface.

### tauri-playwright candidate

From `srsholmes/tauri-playwright` README:

- Rust side:
  - optional dependency `tauri-plugin-playwright`
  - feature gate `e2e-testing = ["tauri-plugin-playwright"]`
  - conditional plugin registration in builder
- JS side:
  - `@srsholmes/tauri-playwright` fixture API
  - Playwright projects can include both `browser` and `tauri` modes from same tests
- capability requirement:
  - add `playwright:default` permission

### Playwright evidence practices

Playwright docs confirm artifact-oriented config pattern is stable:

- `trace: 'on-first-retry'`
- `screenshot: 'only-on-failure'`
- `video: 'retain-on-failure'` / `'on-first-retry'`
- all outputs grouped under configurable `outputDir` for CI artifact packaging

## Compatibility / Risk Analysis

### Key risk 1: test plugin leaks into release builds

- Mitigation:
  - optional dependency + feature gate in Cargo
  - conditional plugin registration in runtime
  - keep release command paths without `--features e2e-testing`

### Key risk 2: fixture drift between old/new suites

- Mitigation:
  - new tauri suite fixtures must reuse semantic contract from:
    - `apps/client/web/app/tests/fixtures/auth.ts`
    - `apps/client/web/app/tests/fixtures/tenant.ts`
    - `e2e-tests/helpers/tenant.mjs`
  - stable tenant identities remain `tenant_a_user` / `tenant_b_user`

### Key risk 3: false pass from partial E2E execution

- Mitigation:
  - explicit “full green” command sequence covering:
    - WDIO suite
    - web Playwright project matrix
    - new tauri-playwright migrated specs
  - CI summary must report all three lanes, not just new lane

### Key risk 4: migration breaks rollback

- Mitigation:
  - no deletion/modification of WDIO spec set beyond non-breaking helper alignment
  - keep WDIO scripts and CI job unchanged in behavior

## Recommended Plan Shape

Use 3 execute plans (2–3 tasks each, ~50% context each):

1. **Plan 01 (Wave 1):** Feature-gated Tauri integration + independent suite scaffold
2. **Plan 02 (Wave 2):** Migrate Phase 1 tests (smoke/login/counter) with fixture parity
3. **Plan 03 (Wave 3):** CI macOS observer lane + full E2E run contract + diagnostics closure

## Validation Architecture

### Trust boundaries

1. Test runner ↔ Tauri automation plugin socket
2. CI workflow ↔ artifact store
3. Auth/tenant fixtures ↔ backend API/middleware

### Security controls to enforce in plan tasks

- Gate plugin registration by `e2e-testing` (or debug assertions as fallback) (mitigate EoP).
- Keep release binary behavior unchanged (mitigate tampering).
- Limit artifact payload to diagnostics only, no secrets (mitigate information disclosure).

### Required automated verification layers

1. **Rust compile gate:** `cargo check -p native-tauri`
2. **New suite smoke gate:** tauri-playwright smoke
3. **Migrated spec gate:** smoke/login/counter in tauri mode
4. **Full E2E gate:** WDIO + web Playwright matrix + tauri-playwright migrated set

## Decision Coverage Confirmation

- D-01/D-02: covered via Phase 1-only migration scope and spec list lock.
- D-03: covered via independent `e2e-desktop-playwright/` suite root.
- D-04: covered via fixture parity tasks against existing auth/tenant semantics.
- D-05/D-06: covered via added macOS observer job while retaining current jobs.
- D-07/D-08: covered via full-green acceptance and artifact output requirements.
- D-09: covered via feature-gated plugin integration and release isolation.
- D-10: covered by explicit “no product behavior change” task constraints.

## Output

Research is sufficient for planning. Proceed to `14-01/02/03-PLAN.md` generation.
