---
phase: 14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs
plan: 05
subsystem: testing
tags: [playwright, tauri, e2e, ci, quality-gate, decommission]

requires:
  - phase: 14-04
    provides: tauri-playwright phase1 baseline and fixture direction
provides:
  - admin/agent/tenant-isolation desktop scenarios migrated to tauri-playwright single stack
  - repository and CI E2E gate reduced from WDIO+web+tauri to web+tauri dual lane
  - planning evidence updated to WDIO decommission reality with D-06 override recorded
affects: [QGATE-01, QGATE-02, phase-14-verification, e2e-gate]

tech-stack:
  added: []
  patterns: [desktop-e2e single stack, deterministic dual-lane gate summary, retired legacy-lane stubs]

key-files:
  created:
    - .planning/phases/14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs/deferred-items.md
    - .planning/phases/14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs/14-05-SUMMARY.md
  modified:
    - e2e-desktop-playwright/tests/specs/admin.spec.ts
    - e2e-desktop-playwright/tests/specs/agent.spec.ts
    - e2e-desktop-playwright/tests/specs/tenant-isolation.spec.ts
    - e2e-desktop-playwright/package.json
    - moon.yml
    - Justfile
    - .github/workflows/e2e-tests.yml
    - e2e-tests/package.json
    - e2e-tests/wdio.conf.mjs
    - e2e-tests/scripts/run-desktop-e2e.mjs
    - .planning/phases/14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs/14-VERIFICATION.md
    - .planning/phases/14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs/14-VALIDATION.md
    - .planning/phases/14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs/14-CONTEXT.md

key-decisions:
  - "按用户约束覆盖 D-06：WDIO 正式退场，桌面 E2E 仅保留 tauri-playwright。"
  - "保留 e2e-tests 目录但将入口/配置标记 retired，避免活动执行路径继续引用旧栈。"

patterns-established:
  - "Desktop migration core pack: test:desktop:core 聚合 smoke/login/counter/admin/agent/tenant-isolation"
  - "Dual-lane full gate: web + tauri 两轨顺序聚合并显式非零退出"

requirements-completed: [QGATE-01, QGATE-02]
duration: 46min
completed: 2026-04-07
---

# Phase 14 Plan 05: Gap Closure Summary

**WDIO desktop lane fully retired while required admin/agent/tenant scenarios were migrated into tauri-playwright with dual-lane diagnosable E2E gating.**

## Performance

- **Duration:** 46 min
- **Started:** 2026-04-07T18:28:44+08:00
- **Completed:** 2026-04-07T19:14:48+08:00
- **Tasks:** 3
- **Files modified:** 13

## Accomplishments

- 完成 Task 1 迁移：`admin`、`agent`、`tenant-isolation` 桌面用例落到 `e2e-desktop-playwright`，并新增 `test:desktop:core` 分组脚本。
- 完成 Task 2 退场：`moon.yml`/`Justfile`/`.github/workflows/e2e-tests.yml` 去除 WDIO 活跃 lane，`e2e-tests` 入口与配置改为 retired。
- 完成 Task 3 文档收敛：14-VERIFICATION / 14-VALIDATION / 14-CONTEXT 同步到“WDIO 已退场”新口径并显式记录 D-06 override。

## Task Commits

1. **Task 1 (TDD RED): add failing migration specs** - `5b64d1f` (test)
2. **Task 1 (TDD GREEN): migrate admin/agent/tenant specs** - `6e47a4e` (feat)
3. **Task 2: retire WDIO lane from repo/CI gate** - `461cbed` (chore)
4. **Task 3: align planning evidence docs** - `43f69c8` (docs)

## Files Created/Modified

- `e2e-desktop-playwright/tests/specs/admin.spec.ts` - 迁移 admin guard/layout/stat 断言到 tauri-playwright。
- `e2e-desktop-playwright/tests/specs/agent.spec.ts` - 迁移 agent guard/layout/input/send state 断言。
- `e2e-desktop-playwright/tests/specs/tenant-isolation.spec.ts` - 迁移 tenant_a_user/tenant_b_user 隔离语义，补 API ready 管理。
- `e2e-desktop-playwright/package.json` - 新增 `test:desktop:core`。
- `moon.yml` - `test-e2e-full` 删除 WDIO lane，`test-desktop` 指向新栈。
- `Justfile` - E2E 命令注释与桌面测试口径更新。
- `.github/workflows/e2e-tests.yml` - 删除 WDIO job，保留 web + tauri artifacts。
- `e2e-tests/package.json` / `wdio.conf.mjs` / `scripts/run-desktop-e2e.mjs` - retired 标记，阻止继续活动执行。
- `14-VERIFICATION.md` / `14-VALIDATION.md` / `14-CONTEXT.md` - 计划证据口径对齐 WDIO retirement。

## Decisions Made

- 用户约束优先，执行 D-06 覆盖：不再维护 WDIO rollback lane green 作为阶段硬要求。
- 为减少误用风险，对 `e2e-tests` 保留目录但使入口显式失败并给出迁移指引。

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] 修复迁移后 admin/agent 断言与实际页面文案/加载时序不一致**
- **Found during:** Task 1
- **Issue:** 初版迁移断言使用了不稳定或不匹配文案，导致假失败。
- **Fix:** 收敛到现有页面稳定可见文案并补最小等待策略。
- **Files modified:** `e2e-desktop-playwright/tests/specs/admin.spec.ts`, `e2e-desktop-playwright/tests/specs/agent.spec.ts`
- **Verification:** `rtk bun run --cwd e2e-desktop-playwright test:ci`（18 passed）
- **Committed in:** `6e47a4e`

**2. [Rule 2 - Missing Critical] 为 tenant-isolation 增加 API ready/start 管理**
- **Found during:** Task 1
- **Issue:** tenant API 在本地未必已启动，导致迁移 spec 出现非业务性 `ECONNREFUSED` 假红。
- **Fix:** 在迁移 spec 内补充 API ready 轮询与按需启动 runtime_server 的守卫逻辑。
- **Files modified:** `e2e-desktop-playwright/tests/specs/tenant-isolation.spec.ts`
- **Verification:** `rtk bun run --cwd e2e-desktop-playwright test:ci --grep tenant` 通过，且全量 `test:ci` 通过。
- **Committed in:** `6e47a4e`

---

**Total deviations:** 2 auto-fixed (1 bug, 1 missing critical)
**Impact on plan:** 均为保障迁移正确性与稳定性的必要修复，无范围蔓延。

## Issues Encountered

- `apps/client/web/app` 现有 web E2E 在 tenant fixture 处出现 `401`（`tenant init failed`），导致 `rtk bun run --cwd apps/client/web/app test:e2e` 与 `rtk just test-e2e-full` 当前非绿。
- 按范围边界，该问题与 14-05 WDIO 退场改动无直接因果，已记录到 `deferred-items.md` 供后续专门计划处理。

## Known Stubs

- `e2e-tests/wdio.conf.mjs`（整文件）: retired stub，明确标记 legacy WDIO 已退场并引导到新套件。
- `e2e-tests/scripts/run-desktop-e2e.mjs`（整文件）: retired stub，执行即失败并提示使用 `e2e-desktop-playwright`。

## Threat Flags

None. 本计划未引入超出 threat_model 的新外部信任边界；变更集中于测试栈迁移与门禁编排收敛。

## Next Phase Readiness

- 桌面 E2E 单栈（tauri-playwright）已具备必要场景覆盖与 CI artifact 产出。
- 后续建议单独处理 web lane 的 tenant init 401 稳定性，以恢复 `test-e2e-full` 全绿。

## Self-Check: PASSED
