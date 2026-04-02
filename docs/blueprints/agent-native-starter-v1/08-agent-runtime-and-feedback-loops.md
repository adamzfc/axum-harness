# 08. Agent Runtime、Skills、MCP 与反馈闭环

## 8.1 为什么 agent 要成为一级架构对象

当 agent 长期参与开发时，它不再只是“写代码工具”，而是仓库参与者。  
既然是参与者，就必须有：

- 可读取的规则
- 可发现的工具
- 可执行的任务
- 可验证的出口
- 可复盘的失败记录

## 8.2 `.agents/` 结构建议

```text
.agents/
  skills/
    rust-core/
    tauri-host/
    sveltekit-ui/
    contracts-typegen/
    tracing-otel/
    testing-playwright/
    protocol-atproto/
    protocol-farcaster/
    chain-evm/
    release-ci/

  prompts/
    add-feature.md
    add-host.md
    refactor-boundary.md
    fix-regression.md
    add-worker.md

  playbooks/
    create-feature.md
    create-adapter.md
    update-contracts.md
    protocol-replay.md
    release-patch.md

  rubrics/
    code-review.md
    boundary-compliance.md
    task-completion.md
    eval-failure-buckets.md
```

## 8.3 skills 的设计原则

每个 skill 至少说明：

- 适用场景
- 输入输出
- 允许改动目录
- 必跑验证
- 常见陷阱
- 失败回滚策略

## 8.4 playbook 的设计原则

playbook 用来规范多步任务。  
例如“新增 feature”至少应包含：

1. 确认目录与命名
2. 更新 contracts
3. 运行 typegen
4. 创建 feature 骨架
5. 添加 adapter 对接
6. 增补 tests
7. 运行 verify
8. 更新 docs/examples

## 8.5 MCP 的职责

你自己的 MCP server 应该提供仓库私有能力，而不是重复 shell。

建议优先建设：

- repo-graph server
- task-registry server
- contract-schema server
- docs-index server
- eval-dataset server
- changelog/release server

## 8.6 反馈闭环

一个完整的 agent 闭环应该是：

`goal -> plan -> tool calls -> patch -> verify -> eval -> result -> rubric -> repo memory`

### 必须沉淀的失败类型

- 目录边界违规
- contracts 不同步
- 测试补漏
- host 污染 core
- worker 绕过 event contracts
- secrets 泄露
- 任务选择错误

## 8.7 规则优先级

建议定义规则层级：

1. Safety / Security
2. Boundary / Architecture
3. Contracts / Typegen
4. Tests / Verify
5. Style / Naming
6. Optional optimization

## 8.8 最小可行 agent 运行层

V1 不需要一开始做超级复杂的多 agent 调度。  
但必须有：

- AGENTS.md
- skills
- playbooks
- rubrics
- eval suites
- task registry
- 可追踪的 patch/verify 记录

## 8.9 何时升级到更复杂的 agent 系统

当出现以下情况时再升级：

- 多种 agent 角色稳定并行
- 仓库复杂度足够高，单个 agent 上下文不够
- regression 和评估数据量明显增长
- protocol/chain/host 维度需要专业 skill 分工
