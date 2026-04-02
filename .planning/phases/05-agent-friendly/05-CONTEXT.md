# Phase 5: Agent-Friendly 开发基建 - Context

**Gathered:** 2026-04-02
**Status:** Ready for planning

<domain>
## Phase Boundary

创建 .agents/ 目录下的 skills、prompts、playbooks、rubrics，让新 agent 能在一小时内通过规则矩阵和 playbook 安全开始工作。

**In scope:**
- .agents/skills/ — 5 个 skills (rust-core, tauri-host, sveltekit-ui, contracts-typegen, testing)
- .agents/prompts/ — 标准 prompt 模板 (add-feature, add-host, refactor-boundary)
- .agents/playbooks/ — 多步任务规范 (create-feature, update-contracts)
- .agents/rubrics/ — 评估标准 (code-review, boundary-compliance, task-completion)

**Out of scope:**
- Skills 和 Prompts 的具体内容 — 用户后续自行补充
- CI 基础设施搭建 — Phase 1 已完成基础，此处只写 rubric 文档

</domain>

<decisions>
## Implementation Decisions

### Skills
- **D-01:** 5 个 skills (rust-core, tauri-host, sveltekit-ui, contracts-typegen, testing) 暂不创建 — 用户后续自行补充
- **D-02:** 已有 `.agents/skills/tailwindcss/SKILL.md` 保留不动

### Prompts
- **D-03:** 标准 prompts (add-feature, add-host, refactor-boundary) 暂不创建 — 用户后续自行补充

### Playbooks
- **D-04:** 混合模式 — 核心流程给高层步骤 (3-5 步概览)，关键路径（如 contracts 变更、边界修改）给详细步骤 + 每步验证命令
- **D-05:** 每个 playbook 包含：触发条件、前置检查、执行步骤、验证步骤、回滚路径
- **D-06:** 关键 playbook (create-feature, update-contracts) 必须包含 `just verify` 和 `just typegen` 验证步骤

### Rubrics
- **D-07:** 混合执行模式 — 可自动化的走 CI (边界检查、类型检查、测试通过)，不可自动化的走 agent review (代码可读性、架构合理性、命名一致性)
- **D-08:** 已有 `.agents/rubrics/boundary-compliance.md` 保留并作为模板
- **D-09:** 新增 code-review rubric 覆盖：命名规范、错误处理、测试覆盖、文档完整性
- **D-10:** 新增 task-completion rubric 覆盖：需求对齐、验证通过、无回归、文档更新

### the agent's Discretion
- Playbook 的具体步骤文案和命令格式
- Rubric 的评分体系 (pass/fail vs 打分制)
- .agents/ 目录内的文件命名约定

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Architecture & Boundaries
- `.agents/rubrics/boundary-compliance.md` — 六边形架构边界规则，已有 rubric 模板
- `docs/blueprints/agent-native-starter-v1/` — 蓝图目录，定义整体架构方向
- `.planning/PROJECT.md` — 项目原则、技术栈决策、Key Decisions 表
- `.planning/REQUIREMENTS.md` — AGENT-DEV-01 需求定义

### Toolchain
- `Justfile` — 顶层命令入口 (setup, dev, verify, typegen)
- `.planning/ROADMAP.md` — Phase 5 目标和 Success Criteria

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `.agents/rubrics/boundary-compliance.md` — 已有的 rubric 模板，可作为 code-review 和 task-completion 的参考结构
- `.agents/skills/tailwindcss/SKILL.md` — 外部安装的 skill，展示 skill 文件格式

### Established Patterns
- 六边形架构边界严格 (PROJECT.md + boundary-compliance.md)
- contracts 作为单一真理源 (Phase 02 validated)
- Just 作为 thin entry point 委托给 moon (Phase 01 validated)
- 测试栈: cargo test + rstest (Rust), Vitest + vitest-browser-svelte (Svelte), Playwright (E2E)

### Integration Points
- Playbooks 需要引用 Justfile 命令 (just verify, just typegen, just dev)
- Rubrics 需要与 CI 边界检查 (repo:boundary-check) 对齐
- Skills 需要覆盖 Phase 1-4 已建立的架构模式

</code_context>

<specifics>
## Specific Ideas

- Playbook 采用混合模式：日常流程 (如 create-feature) 给高层步骤，关键流程 (如 update-contracts) 给详细步骤 + 验证
- Rubric 采用混合模式：边界检查、类型检查走 CI；代码可读性、架构合理性走 agent review
- Skills 和 Prompts 用户后续自行补充，Phase 5 只搭建骨架 + Playbooks + Rubrics

</specifics>

<deferred>
## Deferred Ideas

- Skills 具体内容 (rust-core, tauri-host, sveltekit-ui, contracts-typegen, testing) — 用户后续自行创建
- Prompts 具体内容 (add-feature, add-host, refactor-boundary) — 用户后续自行创建

### Reviewed Todos (not folded)
None — no pending todos matched this phase.

</deferred>

---

*Phase: 05-agent-friendly*
*Context gathered: 2026-04-02*
