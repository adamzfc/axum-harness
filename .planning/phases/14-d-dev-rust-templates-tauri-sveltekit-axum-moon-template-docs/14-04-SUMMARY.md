---
phase: 14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs
plan: 04
subsystem: testing
tags: [tauri, playwright, e2e, capabilities, desktop]

requires:
  - phase: 14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs
    provides: tauri-playwright phase1 migration baseline from plans 01-03
provides:
  - default capability path no longer requires playwright permission
  - e2e-only capability profile and tauri e2e config override
  - stabilized phase1 counter spec with tenant fixture execution path
affects: [QGATE-01, desktop-e2e-playwright-tauri, phase1-specs]

tech-stack:
  added: [none]
  patterns: [tauri capability isolation via profile, guarded locator interactions in flaky ui tests]

key-files:
  created:
    - apps/client/native/src-tauri/capabilities/e2e-testing.json
    - apps/client/native/src-tauri/tauri.e2e.conf.json
  modified:
    - apps/client/native/src-tauri/Cargo.toml
    - Cargo.lock
    - apps/client/native/src-tauri/capabilities/default.json
    - e2e-desktop-playwright/playwright.config.ts
    - e2e-desktop-playwright/tests/specs/counter.spec.ts
    - e2e-desktop-playwright/tests/fixtures/tenant.ts

key-decisions:
  - "Use dedicated e2e capability profile to keep default native build path clean."
  - "Stabilize counter interactions with visible/enabled guards while preserving auth-guard-compatible assertions."

patterns-established:
  - "Tauri E2E capability pattern: default + e2e-testing via tauri.e2e.conf.json"
  - "Counter action pattern: wait for stable controls, then assert value transitions"

requirements-completed: [QGATE-01]
duration: 19 min
completed: 2026-04-07
---

# Phase 14 Plan 04: Capability Isolation & Phase1 Counter Stabilization Summary

**通过 e2e 专用 capability 配置隔离 desktop 自动化权限，并修复 tauri phase1 counter 迁移用例稳定性与租户 fixture 执行接线。**

## Performance

- **Duration:** 19 min
- **Started:** 2026-04-07T16:51:17+08:00
- **Completed:** 2026-04-07T17:10:00+08:00
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- 移除默认 capability 中的 `playwright:default`，新增 `e2e-testing` profile 并通过 `tauri.e2e.conf.json` 组合加载。
- 更新 tauri playwright 启动命令为 `--features e2e-testing --config tauri.e2e.conf.json`，保持默认构建与 e2e 构建均可编译。
- 迁移 counter spec 接入 tenant fixture 执行路径，增加可见/可用守卫与稳定等待策略，`test:phase1` 通过。

## Task Commits

1. **Task 1: Isolate playwright capability to e2e-only profile** - `eed5b29` (feat)
2. **Task 2 (RED): Stabilize phase1 counter spec failing test** - `d1019ef` (test)
3. **Task 2 (GREEN): Implement stabilization + tenant fixture execution wiring** - `d1cc982` (feat)

## Files Created/Modified
- `apps/client/native/src-tauri/capabilities/default.json` - 去除默认 playwright 权限。
- `apps/client/native/src-tauri/capabilities/e2e-testing.json` - 新增 e2e-only playwright capability。
- `apps/client/native/src-tauri/tauri.e2e.conf.json` - 新增 e2e 组合 capability 配置。
- `apps/client/native/src-tauri/Cargo.toml` - 处理 capability 解析阻塞，保证两条编译路径可执行。
- `Cargo.lock` - 对应依赖解析更新。
- `e2e-desktop-playwright/playwright.config.ts` - e2e tauri 启动命令改为显式 e2e 配置。
- `e2e-desktop-playwright/tests/specs/counter.spec.ts` - 引入 tenant fixture 与稳定点击守卫。
- `e2e-desktop-playwright/tests/fixtures/tenant.ts` - 导出 counter controls ready helper，支持迁移 spec 执行路径。

## Decisions Made
- 保持 smoke/login 断言语义不变，仅在 counter spec 和 tenant fixture 上做最小修复，避免改动业务行为。
- tenant reset 在迁移执行路径中执行，但允许在受保护未登录态下平滑退化，确保与 auth guard 语义兼容。

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] 默认构建路径仍因 capability 解析失败**
- **Found during:** Task 1
- **Issue:** 仅拆分 capability 文件后，默认 `cargo check -p native-tauri` 仍报 `Permission playwright:default not found`。
- **Fix:** 调整 `Cargo.toml` 中 `tauri-plugin-playwright` 依赖与 feature 映射，使默认与 e2e 路径均可编译。
- **Files modified:** `apps/client/native/src-tauri/Cargo.toml`, `Cargo.lock`
- **Verification:** `rtk cargo check -p native-tauri` 与 `rtk cargo check -p native-tauri --features e2e-testing` 均通过。
- **Committed in:** `eed5b29`

**2. [Rule 1 - Bug] counter 迁移用例仍有交互抖动失败**
- **Found during:** Task 2
- **Issue:** `test:phase1` 中 counter 交互在窗口按钮竞争/时序下偶发不可见或不可点。
- **Fix:** 在 spec 中引入可见/可用守卫，fixture 导出控制按钮稳定等待 helper，并在 tenant reset 路径中复用。
- **Files modified:** `e2e-desktop-playwright/tests/specs/counter.spec.ts`, `e2e-desktop-playwright/tests/fixtures/tenant.ts`
- **Verification:** `rtk bun run --cwd e2e-desktop-playwright test:phase1` 通过。
- **Committed in:** `d1cc982`

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** 偏差均为闭环必需修复，未引入范围蔓延。

## Issues Encountered
- `favicon.ico` 404 在运行日志中持续出现，但不影响本计划验收命令结果。

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Plan 14-04 目标完成：能力隔离与 phase1 迁移稳定性已具备执行闭环。
- 可进入 14-05 或由 orchestrator 统一做跨 worktree 状态回写。
