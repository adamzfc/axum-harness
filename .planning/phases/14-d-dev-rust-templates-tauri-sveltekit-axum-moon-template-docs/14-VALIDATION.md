---
phase: 14
slug: d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-04-07
---

# Phase 14 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Playwright 1.58.2 + WDIO 9 + Cargo test |
| **Config file** | `apps/client/web/app/playwright.config.ts`, `e2e-tests/wdio.conf.mjs`, `e2e-desktop-playwright/playwright.config.ts` |
| **Quick run command** | `bun run --cwd e2e-desktop-playwright test:smoke` |
| **Full suite command** | `bun run --cwd e2e-tests test:ci && bun run --cwd apps/client/web/app test:e2e && bun run --cwd e2e-desktop-playwright test:ci` |
| **Estimated runtime** | ~1200 seconds |

---

## Sampling Rate

- **After every task commit:** Run `bun run --cwd e2e-desktop-playwright test:smoke`
- **After every plan wave:** Run `bun run --cwd e2e-desktop-playwright test:ci`
- **Before `/gsd-verify-work`:** Full suite must be green
- **Max feedback latency:** 1200 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 14-01-01 | 01 | 1 | QGATE-01 | T-14-01 | e2e plugin only under test feature | compile | `cargo check -p native-tauri` | ✅ | ⬜ pending |
| 14-01-02 | 01 | 1 | QGATE-01 | T-14-02 | tauri smoke can attach without changing product logic | e2e smoke | `bun run --cwd e2e-desktop-playwright test:smoke` | ✅ | ⬜ pending |
| 14-02-01 | 02 | 2 | QGATE-01 | T-14-03 | login and counter migrated with fixture parity | e2e | `bun run --cwd e2e-desktop-playwright test:phase1` | ✅ | ⬜ pending |
| 14-03-01 | 03 | 3 | QGATE-02 | T-14-04 | CI contains macOS tauri observer lane and diagnostics | workflow lint | `grep -n "desktop-e2e-playwright-tauri\|macos-latest\|upload-artifact" .github/workflows/e2e-tests.yml` | ✅ | ⬜ pending |
| 14-03-02 | 03 | 3 | QGATE-01,QGATE-02 | T-14-05 | full repository E2E gate script runs all three tracks | integration | `bun run e2e:full` | ✅ | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Branch protection includes Windows desktop required check and does not remove existing checks | QGATE-01 | GitHub repo settings are dashboard-only | Open GitHub branch protection settings, verify required checks include existing Windows desktop gate and workflow status reflects new observer lane outputs |

---

## Validation Sign-Off

- [x] All tasks have `<automated>` verify or Wave 0 dependencies
- [x] Sampling continuity: no 3 consecutive tasks without automated verify
- [x] Wave 0 covers all MISSING references
- [x] No watch-mode flags
- [x] Feedback latency < 1200s
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
