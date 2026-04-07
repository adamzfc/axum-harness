# Deferred Items (Phase 14.1-02)

## Out-of-scope discoveries during execution

1. **Web E2E full matrix still has pre-existing Safari lane regressions**
   - Command:
     - `rtk bun run --cwd apps/client/web/app test:e2e`
   - Evidence:
     - `[desktop-safari] tests/e2e/agent.test.ts:37` failed: `input[placeholder="Type a message..."]` not found
     - `[desktop-safari] tests/e2e/counter.test.ts:60` failed: `.font-mono` counter locator not found
   - Scope reason:
     - 本计划目标是 preflight/编排与入口一致性，不包含 UI 选择器兼容性修复。
   - Follow-up:
     - 在后续稳定性计划中单独处理 Safari lane 选择器与渲染差异。
