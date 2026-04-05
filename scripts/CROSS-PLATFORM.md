# Cross-Platform Test Scripts

These `.mjs` scripts are cross-platform equivalents of the `.sh` scripts.
They work on Windows, macOS, and Linux using Bun or Node.js.

| Shell Script | Cross-Platform Equivalent |
|-------------|--------------------------|
| `scripts/test/run.sh` | `scripts/test/run.mjs` |
| `scripts/test/run-frontend.sh` | `scripts/test/run-frontend.mjs` |
| `scripts/quick-test.sh` | `scripts/quick-test.mjs` |
| `scripts/test-verify.sh` | `scripts/test-verify.mjs` |

## Usage

```bash
# Using bun (recommended)
bun run scripts/test/run.mjs nextest
bun run scripts/test/run-frontend.mjs all
bun run scripts/quick-test.mjs
bun run scripts/test-verify.mjs

# Using node
node scripts/test/run.mjs nextest
node scripts/test/run-frontend.mjs all
node scripts/quick-test.mjs
node scripts/test-verify.mjs
```

## Commands

### run.mjs
- `nextest` — Run tests with cargo-nextest (default)
- `coverage` — Run tests with cargo-llvm-cov
- `hack` — Run cargo-hack feature powerset check
- `mutants` — Run cargo-mutants mutation testing
- `quick` — Quick smoke test (unit only)
- `all` — Run all test suites

### run-frontend.mjs
- `check` — TypeScript/svelte-check
- `lint` — Biome lint
- `unit` — Vitest unit tests
- `e2e` — Playwright E2E tests
- `all` — Run all frontend checks (default)

### quick-test.mjs
Fast local verification: format, clippy, build, tests, frontend check.

### test-verify.mjs
Full quality gate verification with summary report.
