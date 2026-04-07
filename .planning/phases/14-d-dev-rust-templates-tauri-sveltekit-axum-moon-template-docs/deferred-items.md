# Deferred Items (Phase 14-05)

## Out-of-scope discoveries during execution

1. **Web Playwright E2E currently red due tenant init 401**
   - Command: `rtk bun run --cwd apps/client/web/app test:e2e`
   - Symptom: multiple tests fail at `apps/client/web/app/tests/fixtures/tenant.ts` with `tenant init failed: status=401`
   - Scope reason: this is pre-existing web-lane behavior and not introduced by 14-05 WDIO retirement changes.
   - Suggested follow-up: fix tenant fixture/API auth contract in a dedicated plan for web E2E stability.
