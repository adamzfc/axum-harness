---
phase: 14-d-dev-rust-templates-tauri-sveltekit-axum-moon-template-docs
reviewed: 2026-04-07T11:28:04Z
depth: standard
files_reviewed: 10
files_reviewed_list:
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
findings:
  critical: 0
  warning: 4
  info: 0
  total: 4
status: issues_found
---

# Phase 14: Code Review Report

**Reviewed:** 2026-04-07T11:28:04Z  
**Depth:** standard  
**Files Reviewed:** 10  
**Status:** issues_found

## Summary

本次审查聚焦 Phase 14 的 desktop E2E 迁移与 CI gate 收敛（WDIO 退场、tauri-playwright 接管、双通道门禁）。未发现新的高危安全漏洞或会直接导致生产数据风险的问题。

主要回归风险集中在**测试可靠性与可诊断性**：部分迁移后的桌面用例仍允许“未登录即通过”的路径，存在固定 sleep 导致波动；另外 CI summary 对 matrix 结果呈现不准确，降低失败定位效率。

## Warnings

### WR-01: 认证态断言被“登录页兜底返回”吞掉，降低用例有效性

**File:** `e2e-desktop-playwright/tests/specs/admin.spec.ts:19-27,36-39,50-53,66-69`  
**File:** `e2e-desktop-playwright/tests/specs/agent.spec.ts:17-20,30-33`

**Issue:** 多个“when authenticated”测试在检测到登录页后直接 `return`，导致即使认证态页面回归失效，这些测试也可能通过。这会产生假绿，影响 E2E gate 的真实性。

**Fix:** 将“未认证重定向”与“已认证页面渲染”拆成两类用例；对后者在测试前强制执行 mock 登录（或复用统一 auth fixture），不允许登录页兜底通过。

```ts
test('displays admin dashboard layout when authenticated', async ({ tauriPage, authSession }) => {
  await authSession.loginAs('tenant_a_user');
  await tauriPage.goto(`${APP_BASE_URL}/admin`);
  await expect(tauriPage.getByText('Admin Dashboard')).toBeVisible();
});
```

### WR-02: 固定 sleep 等待导致测试波动与慢机假失败

**File:** `e2e-desktop-playwright/tests/specs/admin.spec.ts:16,34,48`  
**File:** `e2e-desktop-playwright/tests/specs/agent.spec.ts:28`

**Issue:** 使用 `setTimeout(500/600ms)` 做页面稳定等待，容易在 CI 资源波动时触发非业务性失败（过短）或浪费时间（过长）。

**Fix:** 使用可观察条件等待（`expect(locator).toBeVisible()`、`waitForURL`、`waitForLoadState`），替代硬编码睡眠。

```ts
await tauriPage.goto(`${APP_BASE_URL}/agent`);
await expect(tauriPage.getByRole('button', { name: 'New Chat' })).toBeVisible({ timeout: 10_000 });
```

### WR-03: Windows 下 taskkill 未等待完成，可能遗留 runtime_server 进程

**File:** `e2e-desktop-playwright/tests/specs/tenant-isolation.spec.ts:133-136`

**Issue:** `stopOwnedApiProcess()` 在 Windows 分支通过 `spawn('taskkill', ...)` 异步触发后立即返回并清空句柄，未确认进程实际退出。后续测试轮次可能遇到端口占用/僵尸进程，影响稳定性。

**Fix:** 改为同步或可等待的终止方式（`spawnSync`/`once('exit')`），确保清理完成再结束测试。

```ts
import { spawnSync } from 'node:child_process';

spawnSync('taskkill', ['/PID', String(ownedApiProcess.pid), '/F', '/T'], {
  stdio: 'ignore',
  shell: false
});
```

### WR-04: CI Summary 将 web matrix 三平台状态错误复用为同一值

**File:** `.github/workflows/e2e-tests.yml:141-143`

**Issue:** `needs.web-e2e.result` 是整个 matrix job 的聚合结果，却被重复写入 Ubuntu/Windows/macOS 三行。该表格会误导排障（看起来像逐平台状态，实际不是）。

**Fix:** 使用拆分后的独立 job（web-e2e-ubuntu/windows/macos）或在每个 matrix 任务内分别写入带 `matrix.os` 的 summary 记录；不要在汇总页伪造逐平台粒度。

```yaml
# 方案示例：拆分为三个 job，再在 summary 中读取各自 needs.*.result
needs: [web-e2e-ubuntu, web-e2e-windows, web-e2e-macos, desktop-e2e-playwright-tauri]
```

---

_Reviewed: 2026-04-07T11:28:04Z_  
_Reviewer: the agent (gsd-code-reviewer)_  
_Depth: standard_
