---
phase: 03-runtime-boundary-convergence
plan: '02'
subsystem: architecture
tags: [rust, hexagonal-architecture, dependency-boundary, usecases]

requires:
  - phase: 02-contracts-typegen-truth-source
    provides: contracts_api crate as workspace member

provides:
  - usecases crate with enforced hexagonal boundary (no contracts_api dependency)

affects:
  - Phase 04 (minimal feature implementation) — usecases boundary enforced for feature code

tech-stack:
  added: []
  patterns:
    - "Hexagonal boundary enforcement: core layers only depend on domain (ports), not contracts"

key-files:
  modified:
    - packages/core/usecases/Cargo.toml

key-decisions:
  - "usecases 只依赖 domain + 工具 crate，不依赖 contracts_api，强制六角形边界"

patterns-established:
  - "Cargo.toml 依赖清理即边界执法：无代码变更时，删除 Cargo.toml 行即可完成边界收敛"

requirements-completed:
  - RUNTIME-01

duration: 2min
completed: 2026-04-02
---

# Phase 03 Plan 02: Remove contracts_api Dependency from usecases Summary

**从 usecases crate 移除 contracts_api 依赖，强制六角形架构边界：core 层只依赖 domain（Port traits），不依赖 contracts**

## Performance

- **Duration:** 2 min
- **Started:** 2026-04-02T05:50:00Z
- **Completed:** 2026-04-02T05:52:00Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments
- 从 `packages/core/usecases/Cargo.toml` 移除了 `contracts_api = { workspace = true }`
- 验证 `cargo check -p usecases` 编译通过
- 验证 `cargo test -p usecases` 全部 4 个测试通过
- 六角形边界已强制：usecases 只依赖 domain + 工具 crate

## Task Commits

1. **Task 1: Remove contracts_api dependency from usecases** - `c0e6465` (feat)

**Plan metadata:** (to be committed with SUMMARY/STATE/ROADMAP)

## Files Created/Modified
- `packages/core/usecases/Cargo.toml` — 移除 `contracts_api = { workspace = true }` 行

## Decisions Made
- 纯 Cargo.toml 清理，无代码变更。tenant_service.rs 已经只从 domain::ports 导入，不需要 contracts_api。

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- usecases 六角形边界已强制，可以安全地在 Phase 04 中添加新 usecases 代码
- 下一步：继续 Phase 03 剩余计划（RUNTIME-02, RUNTIME-03）

## Self-Check: PASSED

- [x] SUMMARY.md exists at `.planning/phases/03-runtime-boundary-convergence/03-02-SUMMARY.md`
- [x] `packages/core/usecases/Cargo.toml` exists and does NOT contain `contracts_api`
- [x] Commit `c0e6465` exists: `feat(03-02): remove contracts_api dependency from usecases crate`
- [x] Commit `d9c695d` exists: `docs(03-02): complete remove-contracts_api-from-usecases plan`
- [x] cargo check -p usecases passes
- [x] cargo test -p usecases passes (4/4)

---
*Phase: 03-runtime-boundary-convergence*
*Completed: 2026-04-02*
