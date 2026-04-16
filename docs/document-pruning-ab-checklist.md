# A/B 文档保留删除瘦身清单

> 目标：把仓库文档体系收缩为两类高信噪比文档。
>
> A 类：仓库级目标态与规范。
> B 类：局部 owner 文档与 reference chain。
>
> 其余文档如果不能稳定服务 agent 开发，就应瘦身、归档或删除，避免漂移污染上下文。

## 1. 文档分类原则

### 1.1 A 类

定义：

1. 仓库级目标态
2. 规则与边界
3. agent 默认入口
4. 可长期稳定维护
5. 与代码、gate、CI 强相关

### 1.2 B 类

定义：

1. 某个局部目录或局部链路的 owner 文档
2. reference chain
3. 局部运行说明
4. 不重复仓库级规则

### 1.3 应删除或归档的文档

满足以下任一条件就不应作为活文档保留：

1. 重复表达 A 类已有规则
2. 主要是目标态叙事，而非可执行约束
3. 描述不存在的模块或未实现能力
4. 手工维护、极易漂移
5. agent 读了后更容易被误导，而不是更快进入正确惯性

## 2. A 类建议保留清单

以下文档建议保留为活文档，但必须瘦身：

1. `AGENTS.md`
2. `agent/codemap.yml`
3. `agent/manifests/routing-rules.yml`
4. `agent/manifests/gate-matrix.yml`
5. `docs/architecture/repo-layout.md`

### A 类瘦身要求

1. 只写目标态、边界、真理源、顺序、门禁映射。
2. 不再重复解释 DDD 常识和工程常识。
3. 不再承载不存在的引用与过时文件路径。
4. 不再混入“历史计划文档”与“未来扩展散文”。
5. 与代码冲突时必须明确以代码与 validator 为准。

## 3. B 类建议保留清单

以下文档建议保留为活文档：

1. `docs/operations/counter-service-reference-chain.md`
2. `packages/contracts/STRUCTURE.md`
3. `platform/model/README.md` 或并入 `repo-layout.md`
4. `services/*/README.md`
5. `workers/*/README.md`
6. `infra/local/README.md`
7. 必要的 `ops/runbooks/**`

### B 类约束

1. 只写局部 owner 视角。
2. 不重复仓库级规范。
3. 必须标明 `reference`、`implemented`、`stub`、`planned` 状态。
4. 不得把未来态写成当前已实现事实。
5. 必须能帮助 agent 快速进入局部目录开发，而不是提供宏观愿景。

## 4. 当前文档处置建议

### 4.1 保留并重写

1. `AGENTS.md`
2. `agent/codemap.yml`
3. `docs/architecture/repo-layout.md`
4. `docs/operations/counter-service-reference-chain.md`
5. `packages/contracts/STRUCTURE.md`
6. `infra/local/README.md`

### 4.2 保留但降级为按需阅读

1. `platform/model/README.md`
2. `services/*/README.md`
3. `workers/*/README.md`
4. `ops/README.md`
5. `scripts/README.md`

### 4.3 迁移到 archive 或删除

以下文档不建议继续作为活文档：

1. `docs/architecture/context/**`
2. `docs/architecture/container/**`
3. `docs/architecture/component/**`
4. `docs/architecture/sequence/**`
5. `docs/architecture/sync-flow/**`
6. `docs/architecture/topology/**`
7. `docs/architecture/deployment/**`
8. `docs/architecture/maturity-levels.md`
9. `docs/contracts/**` 下大多数手写参考文档
10. `docs/counter-service-gap-fix-plan.md`

## 5. 清理动作清单

### 5.1 第一批

1. 修复 A 类文档中的失效引用。
2. 为所有保留文档标注其角色：A 类或 B 类。
3. 删除 A 类内部重复段落。
4. 将明显的计划文档与 C4 风格图文移出默认路径。

### 5.2 第二批

1. 新建 `docs/README.md` 作为唯一文档入口。
2. 在 `docs/README.md` 中明确：默认只读 A 类与少数 B 类。
3. 将历史文档迁移到 `docs/archive/`。
4. 删除已经被代码、schema、validator 完全替代的散文文档。

### 5.3 第三批

1. 对每个 B 类 README 增加状态标签：`reference`、`implemented`、`stub`、`planned`。
2. 对所有局部 README 统一模板：职责、边界、当前状态、入口、不要做什么。
3. 把可机器化的说明继续下沉为 schema、validator、scripts。

## 6. 文档模板要求

### 6.1 A 类模板要求

应包含：

1. 目标
2. 边界
3. 真理源
4. 规则
5. 变更顺序
6. 与 gate/CI 的关系

不应包含：

1. 长篇背景故事
2. 宏大未来蓝图
3. 与其他 A 类重复的规范
4. 仅给人类看的空泛建议

### 6.2 B 类模板要求

应包含：

1. 该目录/链路负责什么
2. 当前状态
3. 入口文件或入口路径
4. 如何验证
5. 不该做什么

不应包含：

1. 仓库级原则复述
2. 与 A 类重复的架构规范
3. 过度宏观描述

## 7. 验收标准

完成清理后，应满足：

1. agent 默认只需要阅读极少文档即可入场。
2. 文档总量显著下降，但高价值信息密度上升。
3. 不再存在大量“目标态冒充现状”的活文档。
4. A 类只讲规则与目标态，B 类只讲局部 owner 与 reference。
5. 其余历史材料不再污染默认上下文。

## 8. 建议最终结构

建议最终 docs 主干为：

1. `docs/README.md`
2. `docs/architecture/repo-layout.md`
3. `docs/operations/counter-service-reference-chain.md`
4. `docs/backend-infrastructure-audit-checklist.md`
5. `docs/counter-service-reference-chain-checklist.md`
6. `docs/document-pruning-ab-checklist.md`
7. `docs/gate-ci-decoupling-checklist.md`

历史资料可移至：

1. `docs/archive/**`
