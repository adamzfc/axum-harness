# E2E Migration: WDIO → Tauri Playwright

## Context

Desktop E2E tests were migrated from WebDriverIO (WDIO) to Tauri Playwright in Phase 14.
The legacy `e2e-tests/` directory has been retired.

## Migration Summary

**From:** `e2e-tests/` (WDIO + tauri-driver)
**To:** `e2e-desktop-playwright/` (Tauri Playwright)

### What Changed

1. **Test Framework:** WDIO/Mocha → Playwright
2. **Execution:** `tauri-driver` + WebDriver → Direct Tauri Playwright integration
3. **CI Workflow:** `.github/workflows/e2e-tests.yml` updated to use `e2e-desktop-playwright`
4. **Commands:** All `bun run --cwd e2e-tests` commands replaced with `e2e-desktop-playwright` equivalents

### Active E2E Locations

- **Web E2E:** `apps/client/web/app/tests/` (Playwright)
- **Desktop E2E:** `e2e-desktop-playwright/` (Tauri Playwright)
- **Shared helpers:** Migrated into each test suite

### Key Commands

```bash
# Web E2E
just test-e2e

# Desktop E2E (full build + test)
just test-desktop

# Desktop E2E (fast, cached build)
just test-desktop-fast

# Full E2E gate (web + desktop)
just test-e2e-full
moon run repo:test-e2e-full
```

## Troubleshooting

### Desktop E2E fails to start

1. Ensure binary is built: `just test-desktop` (auto-builds via moon dependency)
2. Check build profile: `cargo build -p native-tauri --profile e2e --features e2e-testing`
3. Verify e2e-desktop-playwright dependencies: `cd e2e-desktop-playwright && bun install`

### Web E2E flakiness

- See `.agents/playbooks/debugging/e2e-flakiness.md` (if created)
- Common causes: port conflicts, SvelteKit type generation timing

## Historical Notes

- WDIO required `tauri-driver` installation and complex WebDriver setup
- Windows needed matching `msedgedriver.exe` on PATH
- Tauri Playwright provides direct integration without WebDriver layer
- Migration completed: Phase 14 (2026-04-07)

## References

- Original context: `docs/TAURI_PLAYWRIGHT_MIGRATION_CONTEXT.md`
- QA/UAT procedures: `qa-uat/README.md`
- Phase 14 summaries: `.planning/phases/14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs/`
