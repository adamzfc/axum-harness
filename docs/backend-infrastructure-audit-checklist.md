# 后端基建审计清单

> 目标：在不看 `apps/**` 的前提下，把后端开发轨道收敛为一套可被 agent 稳定复用、可被 gate/CI 强制执行、符合 2026-04 最佳实践的 monorepo admission baseline。

## 1. 文档定位

本清单只审计后端外围基建，不直接审计业务功能正确性。

覆盖范围：

1. `mise`
2. `moon`
3. `justfile` 与 `justfiles/**`
4. `scripts/**`
5. `.github/workflows/**`
6. `Cargo.toml` workspace 与 crate 拓扑
7. `agent/**`
8. `platform/validators/**`

不覆盖范围：

1. `apps/**`
2. 前端构建链路
3. UI 测试
4. 桌面端体验细节

## 2. 目标态基线

后端外围基建应满足以下原则：

1. `mise` 只负责工具链版本与本地环境统一。
2. `moon` 只负责 task graph、affected execution、缓存、并行、分层编排。
3. `just` 只负责人类和 agent 的稳定命令入口，不承载复杂业务逻辑。
4. `scripts/**` 只负责跨平台校验、诊断、编排逻辑。
5. GitHub Actions 只负责 CI orchestration，不重复定义 repo 内规则。
6. `agent/**` 是 agent 路由、边界、门禁映射的真理源。
7. gate 是仓库内部能力，CI 只是调用 gate，不再自建第二套规则。
8. 所有命名按 DDD 分类收口，不能混成“历史习惯命名”。

## 3. 现状问题摘要

当前已确认的后端基建问题：

1. `AGENTS.md` 与 planner skill 存在失效引用，默认必读集不可靠。
2. `just`、`moon`、文档之间有命令名漂移。
3. GitHub Actions 内仍然内嵌大量 package 集与流程判断，gate 与 CI 尚未完全解耦。
4. 规则分层不够统一，存在 `quality`、`platform`、`drift`、`scripts`、`primary/secondary` 等多套命名语言并存。
5. `codemap.yml` 部分内容已与实际代码结构漂移。
6. 2026-04 工具分工意图已经出现，但尚未完全收敛为“单一职责、单一入口、单一真理源”。

## 4. 审计主清单

### 4.1 工具链层

1. 审计 `.mise.toml` 是否只声明工具链版本，不承载业务逻辑。
2. 审计 `rust-toolchain.toml`、`Cargo.toml`、`package.json`、`moon.yml` 的版本策略是否一致。
3. 审计 2026-04 基线工具是否明确固定：Rust、Bun、Moon、Just、Node。
4. 审计本地开发是否存在多套互相冲突的安装入口。
5. 审计 `.env`、SOPS、环境变量加载方式是否在后端层明确分工。

### 4.2 `moon` 编排层

1. 审计 `moon.yml` 是否只定义 task graph，而不直接承载过多业务判断。
2. 审计 `moon` task 命名是否可与 DDD 分类对齐。
3. 审计 `moon` 是否已经成为本地与 CI 可共用的执行图真理源。
4. 审计 `moon` 中是否仍直接耦合 `apps/**` 相关任务，污染后端 admission。
5. 审计 `moon` 的 `dev-*`、`test-*`、`quality-*`、`contracts-*` 是否存在重复表达。

### 4.3 `just` 入口层

1. 审计 `just` 是否只提供稳定别名，而不再内嵌复杂校验逻辑。
2. 审计 `justfiles/**` 的命令命名是否与 `moon`、scripts、文档一致。
3. 审计是否存在文档引用了不存在的 `just` 命令。
4. 审计是否存在 `just` 命令与 `moon` 任务一对多或多对一的混乱关系。
5. 审计 `just` 是否已按后端 admission 视角组织：contracts/services/servers/workers/platform/boundaries/drift。

### 4.4 `scripts/**` 校验层

1. 审计 `scripts/**` 是否是规则执行的主要载体，而不是散落到 workflow 和 just 中。
2. 审计 `scripts/validate-*.ts` 是否覆盖关键后端约束：存在性、导入边界、状态语义、workflow、contract drift、handoff。
3. 审计脚本命名是否与 gate matrix 和 just 命令一致。
4. 审计脚本是否跨平台，避免 shell-only 逻辑成为默认路径。
5. 审计脚本错误输出是否足够 agent 消费，不只是给人类看。

### 4.5 Cargo workspace 与 crate 拓扑

1. 审计 workspace members 是否只包含真实 crate，不保留长期无意义 stub。
2. 审计 crate 命名是否统一反映 DDD 角色，不混入历史命名。
3. 审计 services、servers、workers、validators、packages 的依赖方向是否可被静态校验。
4. 审计 workspace 级 lint 策略是否仍过于宽松，导致 skeleton 长期不收敛。
5. 审计 adapter crate、runtime crate、contracts crate 是否符合“trait/port 与 adapter 分离”的目标态。

### 4.6 `agent/**` 规则层

1. 审计 `codemap.yml` 是否只保留已验证的事实约束与目标态硬规则。
2. 审计 `routing-rules.yml` 是否与当前目录结构一致。
3. 审计 `gate-matrix.yml` 是否能映射到真实存在的 gate 命令。
4. 审计 `agent/**` 是否存在重复散文、无脚本消费价值的说明。
5. 审计 agent 规则是否默认后端优先，避免 `apps/**` 噪音进入主上下文。

### 4.7 GitHub Actions 与 CI 编排

1. 审计 `.github/workflows/**` 是否只是调用 repo 内 gate，而不是重新定义规则。
2. 审计 `ci.yml`、`quality-gate.yml`、`coverage.yml` 的职责边界是否清晰。
3. 审计 workflow 中的 package 列表是否应迁移到 repo 内脚本或 moon 配置。
4. 审计 CI 是否与本地 gate 命令保持同源执行路径。
5. 审计 coverage 是否属于独立信号，而不是 admission 主 gate。

## 5. 收敛改造清单

### 5.1 第一批必须完成

1. 清理失效文档引用与默认必读列表。
2. 统一 `just`、`moon`、scripts、文档的命令命名。
3. 将 GitHub Actions 改造成“只调 gate 命令”的薄编排层。
4. 收敛后端 admission 命令语言：`contracts`、`services`、`servers`、`workers`、`platform`、`boundaries`、`drift`。
5. 标记并隔离前端相关 task，避免进入后端主 admission。

### 5.2 第二批应完成

1. 把 package 列表和范围选择从 workflow 内抽到 repo 内脚本或 moon 任务。
2. 收紧 workspace lint 基线，逐步减少 `allow`。
3. 明确本地 gate、CI gate、scoped gate 的同源关系。
4. 让 `agent/gate-matrix.yml` 与 `scripts/run-scoped-gates.ts` 完全对齐。
5. 为 agent 输出一套后端唯一 admission 入口命令。

### 5.3 第三批可优化

1. 引入 affected backend execution，减少无关 crate 的全量跑批。
2. 引入更精确的 crate 集自动发现，而不是手工 package 列表。
3. 补充 validator 输出结构化 JSON 结果，增强 agent 可消费性。

## 6. 验收标准

完成本清单后，应满足：

1. agent 不需要读大量散文也能知道后端怎么验证。
2. 本地与 CI 跑的是同一套 gate。
3. GitHub Actions 不再自带第二套规则语言。
4. `just`、`moon`、scripts、文档命名一致。
5. 后端 admission 与前端 admission 解耦。
6. DDD 分类能直接体现在 gate 命令、脚本、workflow 中。

## 7. 交付产物

建议最终交付以下产物：

1. 一套精简后的 `AGENTS.md`。
2. 一套事实化、瘦身后的 `agent/codemap.yml`。
3. 一套按 DDD 分类重命名后的 `justfiles/**` 命令入口。
4. 一套与 gate matrix 对齐的 `scripts/**` 校验层。
5. 一套薄编排、可解释、只调用 repo gate 的 GitHub Actions。
