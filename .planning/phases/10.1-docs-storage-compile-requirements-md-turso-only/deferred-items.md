# Deferred Items — 10.1-03

- Task 2 verification blocker: `moon run repo:test-desktop` fails in current environment due to pre-existing desktop Playwright/Tauri harness issues (webServer startup/socket/timeout), not caused by this plan's Turso-only gate config changes.
  - Evidence:
    - `Error: Process from config.webServer was not able to start. Exit code: 127`
    - `Error: Timed out waiting 600000ms from config.webServer`
    - `Error: connect ENOENT ./test-results/tauri-playwright.sock`
  - Scope handling: kept Task 2 changes limited to `moon.yml` and `.github/workflows/e2e-tests.yml`; no cross-cutting harness refactor in this plan.
