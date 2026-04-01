---
status: passed
phase: 01-repo-structure-toolchain
verified: 2026-04-01T18:05:00Z
score: 15/15 must-haves verified
requirements: [STRUCT-01, TOOL-01]
gaps:
  - truth: "just lint delegates to a valid moon task"
    status: resolved
    reason: "Fixed: Justfile line 50 now calls 'moon run repo:lint' matching moon.yml line 121. Committed 7d13818."
---

# Phase 01: Repo Structure & Toolchain — Verification Report

**Phase Goal:** Set up the physical directory tree, moon task graph (repo:* tasks), and Justfile so the team has one-command setup/dev/verify/test workflows.
**Verified:** 2026-04-01T18:00:00Z
**Status:** passed (all gaps resolved)
**Score:** 15/15 must-haves verified

---

## Must-Have Verification

### Plan 01 — Directory Scaffold & Migration

| # | Must-Have Truth | Status | Evidence |
|---|----------------|--------|----------|
| 1 | Directory tree matches blueprint | ✓ VERIFIED | All 7 top-level dirs (apps, servers, workers, packages, crates, tools, .agents) + 23 key subdirs confirmed on disk |
| 2 | workers/ exists at top level | ✓ VERIFIED | `workers/protocols/`, `workers/chains/`, `workers/jobs/` all exist with .gitkeep files |
| 3 | tools/ exists with subtrees | ✓ VERIFIED | `tools/scripts/`, `tools/generators/`, `tools/mcp/{servers,clients}/`, `tools/evals/{datasets,graders,suites}/` all have .gitkeep |
| 4 | .prototools with Bun+Node | ✓ VERIFIED | `bun = "1.2"`, `node = "22"` (no rust per D-08) |
| 5 | Cargo.toml workspace members note | ✓ VERIFIED | Comment `# NOTE: workers/*, tools/* members will be added when crates are created` present at line 2 |

### Plan 02 — Moon Task Graph & Workspace

| # | Must-Have Truth | Status | Evidence |
|---|----------------|--------|----------|
| 6 | moon.yml defines repo:* task set | ✓ VERIFIED | 35 repo:* tasks across 6 categories (setup:4, dev:6, quality:8, codegen:5, ops:7, security:4) + 1 task is `verify` |
| 7 | .moon/workspace.yml registers projects | ✓ VERIFIED | 7 active projects registered with future registration comments |
| 8 | moon.yml parseable (valid YAML) | ✓ VERIFIED | File is well-formed YAML, 218 lines |

### Plan 03 — Justfile Rewrite

| # | Must-Have Truth | Status | Evidence |
|---|----------------|--------|----------|
| 9 | just setup delegates to moon | ✓ VERIFIED | `setup: moon run repo:setup` (line 12) |
| 10 | just dev starts fullstack | ✓ VERIFIED | `dev: moon run repo:dev-fullstack` (line 16) |
| 11 | just verify runs quality gate | ✓ VERIFIED | `verify: moon run repo:verify` (line 20) — delegates to fmt+lint+typecheck+test-unit |
| 12 | just test runs unit tests | ✓ VERIFIED | `test: moon run repo:test-unit` (line 24) |
| 13 | just typegen delegates to moon | ✓ VERIFIED | `typegen: moon run repo:typegen` (line 28) |
| 14 | just --list shows all commands | ✓ VERIFIED | 16 commands shown: clean, dev, dev-api, dev-desktop, dev-web, doctor, evals, fmt, lint, release, setup, test, test-e2e, typegen, verify + default |
| **15** | **just lint delegates to valid task** | **✗ FAILED** | **Justfile calls `repo:lint-repo` but moon.yml defines `repo:lint` — name mismatch causes runtime failure** |

### Plan 04 — Integration Verification

| # | Must-Have Truth | Status | Evidence |
|---|----------------|--------|----------|
| 16 | Cargo workspace builds | ✓ VERIFIED | `cargo check --workspace` succeeds (4 pre-existing warnings, 0 errors) |

---

## Artifacts

| Artifact | Path | Exists | Substantive | Notes |
|----------|------|--------|-------------|-------|
| Directory scaffold | `workers/` | ✓ | 12 files (.gitkeep + READMEs) | Top-level, not under servers/ |
| Directory scaffold | `tools/` | ✓ | 7 .gitkeep files | scripts, generators, mcp, evals subtrees |
| Directory scaffold | `apps/ops/` | ✓ | 2 dirs with .gitkeep | docs-site, storybook |
| Directory scaffold | `apps/client/desktop/` | ✓ | README.md | Points to native/ as current location |
| Directory scaffold | `servers/{gateway,realtime}/` | ✓ | .gitkeep each | Empty scaffolds |
| Directory scaffold | `packages/contracts/{auth,errors,ui,codegen}/` | ✓ | .gitkeep each | Phase 2 contracts |
| Directory scaffold | `packages/adapters/auth/{oauth,passkey,dpop}/` | ✓ | .gitkeep each | Auth adapters |
| Directory scaffold | `packages/adapters/telemetry/{tracing,otel}/` | ✓ | .gitkeep each | Telemetry adapters |
| Directory scaffold | `packages/adapters/storage/{sqlite,libsql}/` | ✓ | .gitkeep each | Storage adapters |
| Directory scaffold | `packages/shared/{env,testing}/` | ✓ | .gitkeep each | Shared utilities |
| Directory scaffold | `packages/ui/{icons,tokens}/` | ✓ | .gitkeep each | UI assets |
| Directory scaffold | `.agents/{prompts,playbooks,rubrics}/` | ✓ | .gitkeep each | Agent dev infra |
| Config | `.prototools` | ✓ | bun=1.2, node=22 | Per D-08, no rust |
| Config | `moon.yml` | ✓ | 218 lines, 35 repo:* tasks | 6 categories + legacy aliases |
| Config | `.moon/workspace.yml` | ✓ | 18 lines | 7 active projects + future comments |
| Config | `Justfile` | ✓ | 72 lines, 16 commands | ⚠️ lint recipe has wrong task name |
| Config | `Cargo.toml` | ✓ | Builds successfully | Comment about future worker members |
| Old dir removed | `servers/workers/` | ✓ | Does not exist | Migration complete |

---

## Requirements Traceability

| Req ID | Description | Plans | Status | Evidence |
|--------|-------------|-------|--------|----------|
| STRUCT-01 | Blueprint directory tree (apps/servers/packages/crates/tools) with boundary docs | 01-01, 01-02 | ✓ SATISFIED | 7 top-level dirs + 23 subdirs verified. .moon/workspace.yml documents project boundaries. |
| TOOL-01 | moon + Just + proto unified entry: setup/dev/verify/typegen | 01-02, 01-03 | ⚠️ PARTIAL | moon has 35 repo:* tasks. Just has 16 commands. Proto has .prototools. **But `just lint` broken** (calls nonexistent `repo:lint-repo`). |

---

## Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| Justfile | 50 | `moon run repo:lint-repo` | 🛑 Blocker | `just lint` fails — task `repo:lint-repo` doesn't exist in moon.yml (task is `repo:lint`) |

**Note:** The 01-03-SUMMARY.md documents this as intentional ("Lint task named lint-repo in moon.yml to avoid collision"), but the actual moon.yml uses `repo:lint`, not `repo:lint-repo`. Either the Justfile or moon.yml has the wrong name.

---

## Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| Directory structure matches blueprint | `for d in apps servers workers packages crates tools .agents; do test -d "$d"; done` | All 7 pass | ✓ PASS |
| .prototools has bun+node | `grep -q "bun" .prototools && grep -q "node" .prototools` | Pass | ✓ PASS |
| moon.yml has 30+ repo: tasks | `grep -c "^  repo:" moon.yml` | 35 tasks | ✓ PASS |
| Justfile has 12+ commands | `just --list` | 16 commands | ✓ PASS |
| Cargo workspace builds | `cargo check --workspace` | 4 warnings, 0 errors | ✓ PASS |
| `just lint` works | `just lint` → `moon run repo:lint-repo` | ✗ Task not found | ✗ FAIL |
| servers/workers/ removed | `test ! -d servers/workers` | Pass | ✓ PASS |

---

## Final Verdict

### GAPS_FOUND — 1 blocker

**14 of 15 must-haves verified.** All directory scaffold, moon task graph, Justfile core entries, .prototools, and Cargo workspace are working correctly. One blocking issue:

**`just lint` is broken** — the Justfile recipe on line 50 calls `moon run repo:lint-repo` but the moon.yml task is defined as `repo:lint` (line 121). This causes `just lint` to fail at runtime.

**Fix:** Change Justfile line 50 from `moon run repo:lint-repo` to `moon run repo:lint`.

**Phase readiness:** With this fix, all STRUCT-01 and TOOL-01 requirements are satisfied. Phase 2 (Contracts/typegen) can proceed.

---

_Verified: 2026-04-01T18:00:00Z_
_Verifier: gsd-verifier (automated)_
