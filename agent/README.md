# Agent Harness

`agent/` 只保留最小的 agent 协作真理源，不承载业务逻辑、模板、checklist 或流程散文。

## 目录职责

1. `codemap.yml`
   负责模块边界、写权限、依赖方向、分布式硬约束与必填语义。
2. `manifests/routing-rules.yml`
   负责 touched paths 到 subagent 的路由与派发顺序。
3. `manifests/gate-matrix.yml`
   负责 subagent 对应的 scoped gates 与最终总验证。

## 使用顺序

1. 先读根级 `AGENTS.md`。
2. 再读 `docs/architecture/repo-layout.md`，理解目标态结构与后端分布式边界。
3. 再读 `agent/codemap.yml`，确认当前任务的边界、禁止事项与 required fields。
4. 最后根据 `routing-rules.yml` 和 `gate-matrix.yml` 决定派发与验证。

## 说明

1. 详细 subagent 行为定义仍在 `.agents/skills/*/SKILL.md`。
2. 参考实现与真实开发模式应优先从现有 `services/*`、`workers/*`、`servers/*` 和 `packages/contracts/*` 获取，而不是依赖 agent 模板。
3. 如果 `agent/` 文档与代码冲突，以代码和可执行验证结果为准。
