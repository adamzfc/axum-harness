# Deferred Items (Phase 14-05)

## Out-of-scope discoveries during execution

1. **Web Playwright E2E red in prior-phase regression gate**
   - Commands:
     - `rtk bun run --cwd apps/client/web/app test:e2e --project=desktop-chrome tests/e2e/counter.test.ts tests/e2e/tenant-isolation.test.ts`
     - `rtk bun run --cwd apps/client/web/app test:e2e`
   - Evidence:
     - Initial failure: `connect ECONNREFUSED 127.0.0.1:3001` during `POST /api/tenant/init`
     - After explicitly starting `runtime_server`: `tenant init failed: status=401`
   - Impact:
     - `counter.test.ts` and `tenant-isolation.test.ts` fail in `beforeEach` fixture setup, so prior-phase regression gate cannot pass.
   - Scope reason:
     - This is a web-lane contract/alignment issue across existing fixture/auth/runtime assumptions, not introduced by 14-05 WDIO retirement itself.
   - Suggested follow-up:
     - Open a dedicated gap/polish phase for web E2E fixture/auth contract stabilization.

2. **E2E startup instability and orchestration friction (non-functional)**
   - Evidence:
     - `Port 5173 is already in use` occasionally blocks Playwright webServer startup.
     - `.svelte-kit/types/src/routes/$types.d.ts` missing in some startup paths until SvelteKit sync completes.
   - Impact:
     - Increases false negatives and execution variance during regression and CI-local reproduction.
   - Suggested follow-up:
     - Add deterministic preflight for web E2E (type generation + port/process hygiene checks).

3. **Full E2E runtime cost is high due repeated Rust compile/start**
   - Evidence:
     - Starting `runtime_server` from cold state took about 8+ minutes before web E2E could run.
   - Impact:
     - Greatly increases iteration cost when running full-gate repeatedly.
   - Suggested follow-up:
     - Create a dedicated performance phase for E2E execution efficiency (warm/cached runtime strategy, prebuilt binary reuse, and lane orchestration optimization).
