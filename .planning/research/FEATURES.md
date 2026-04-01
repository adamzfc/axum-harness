# Feature Research

**Domain:** v0.1.1（架构收敛、决策沉淀与生产闭环）里程碑特性范围
**Researched:** 2026-04-01
**Confidence:** HIGH（基于项目内文档与当前里程碑上下文）

## Feature Landscape

> 本文仅覆盖本里程碑新增能力与“用户可感知的工程结果”（主要是模板消费者/后续 agent 的可感知结果），不重复既有脚手架能力。

### Table Stakes (Users Expect These)

缺失这些，会直接导致“这个模板还不能放心用于生产迭代”。

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **F-TS-01 安全基线闭环（JWT 校验链路最小可用）** | 当前上下文已明确存在 `dangerous::insecure_decode` 历史决策；v0.1.1 必须把“可发布最小安全线”落地 | MEDIUM | 原子验收：关键入口不再依赖 payload-only 解码；失败路径统一返回 401/403；有回归测试。 |
| **F-TS-02 敏感配置治理（env/secret 分级 + fail-fast）** | 生产模板默认应避免“缺配置仍启动” | LOW | 原子验收：缺少关键 secret 启动即失败；示例配置不泄露真实值；文档标注必填/可选。 |
| **F-TS-03 路径可移植性（跨平台路径策略统一）** | 目标是跨平台模板，路径硬编码会直接破坏 Windows/macOS/Linux 一致性 | LOW | 原子验收：数据/缓存/日志路径统一通过平台 API 解析；禁用硬编码绝对路径。 |
| **F-TS-04 Rust/TS 契约自动同步（typegen 可重复执行）** | 里程碑目标已写明“契约与类型闭环”；否则类型漂移持续发生 | MEDIUM | 原子验收：`contracts_api` 作为唯一契约源；`typegen` 一键生成并可校验 dirty diff。 |
| **F-TS-05 运行时边界收敛（runtime_tauri 职责落地）** | 当前痛点是 native host 逻辑漂移；必须恢复清晰边界以降低后续认知成本 | MEDIUM | 原子验收：Tauri 命令入口收敛到 `runtime_tauri`；host 仅保留启动/装配；边界文档可追踪。 |
| **F-TS-06 统一任务入口（fullstack:dev / typegen / verify）** | 模板用户和 agent 需要“一条命令跑起来/校验” | LOW | 原子验收：任务可在本地直接运行；失败信息可定位到阶段（类型/测试/安全）。 |
| **F-TS-07 全量建议决策账本（含 deferred 与 future 摘要）** | 本里程碑核心目标之一就是“决策沉淀” | MEDIUM | 原子验收：每条建议具备状态（implement/defer/out）、原因、目标 phase、追踪链接。 |

### Differentiators (Competitive Advantage)

这些不是“能不能用”的底线，但会显著提升该模板在工程治理上的竞争力。

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **F-DIFF-01 决策账本可执行化（决策→任务→验证 三向映射）** | 不止“记录”，还能直接驱动 roadmap/phase 执行，减少重复讨论 | MEDIUM | 建议在账本中增加 requirement/phase/验证脚本字段，支持 agent 自动消费。 |
| **F-DIFF-02 契约漂移门禁（CI 或 verify 阶段强校验）** | 把类型同步从“约定”升级为“门禁”，长期降低回归成本 | MEDIUM | `typegen` 后有未提交差异则失败；防止 Rust/TS 接口悄然分叉。 |
| **F-DIFF-03 安全基线可观测（最小安全检查清单 + 结果文件）** | 让模板使用者看到“已硬化到什么程度”，增强可审计性 | LOW | 输出 machine-readable 安全检查结果（例如 verify 产物）。 |
| **F-DIFF-04 未来 phase 摘要模板化（defer 也可追踪）** | deferred 事项不会在迭代中遗失，减少里程碑切换信息损失 | LOW | 每个 defer 条目必须包含触发条件（何时升级为 implement-now）。 |

### Anti-Features (Commonly Requested, Often Problematic)

本里程碑明确“不做”，以避免偏离“最小改动+架构收敛”主线。

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| **AF-01 全量安全体系重构（一次性引入复杂 RBAC/JWKS 平台化）** | “既然做安全就一步到位” | 超出 v0.1.1 收敛目标，极易造成范围爆炸与回归风险 | 先完成最小可发布基线；高级安全能力进后续 phase。 |
| **AF-02 全仓大重构（按理想架构重排目录）** | “顺手把历史包袱一起清掉” | 与“最小改动原则”冲突，会拖慢交付并放大冲突面 | 仅做边界收敛所需局部重排，保留兼容层。 |
| **AF-03 新增重型基础设施（额外服务编排/新中间件）** | “把未来可能需要的都预埋” | 该里程碑目标是闭环而非扩张，新增依赖提升维护负担 | 维持现有栈，只补足 `fullstack:dev/typegen/verify`。 |
| **AF-04 以文档替代验证（只写决策不加检查）** | “先记下来以后再说” | 账本无法约束真实行为，后续仍会漂移 | 决策条目必须绑定可执行验证点或验收脚本。 |

## Milestone Scope Breakdown (for requirements scoping)

### Implement Now（v0.1.1 必做）

| ID | Item (Atomic) | Testable Acceptance | Dependencies |
|----|---------------|---------------------|--------------|
| IN-01 | JWT 最小安全校验替换 payload-only 路径 | 未签名/篡改 token 被拒绝；现有鉴权流回归通过 | 现有 auth middleware、测试框架 |
| IN-02 | 敏感配置分级与 fail-fast | 缺关键配置启动失败并给出明确错误 | 配置加载层 |
| IN-03 | 跨平台路径策略统一 | Windows/macOS/Linux 路径构造无硬编码；相关测试通过 | runtime 初始化路径入口 |
| IN-04 | `contracts_api` 成为单一契约源 | 契约定义变更可触发 TS 类型更新 | shared_contracts/contract crate 现状 |
| IN-05 | `typegen` 任务落地并纳入 `verify` | 运行 `typegen` 后无脏 diff 才可通过 verify | Moon/Just 任务体系 |
| IN-06 | `runtime_tauri` 与 host 职责收敛 | host 中业务逻辑减少到装配层；命令边界清晰 | 现有 runtime_server/runtime_tauri 结构 |
| IN-07 | 决策账本 v1（含实现/延期/拒绝 + 原因） | 任一建议条目可回答“是否做、何时做、为何做” | 历史 phase 文档与建议来源 |
| IN-08 | Future phase 简要摘要附着到账本 | deferred 项都有 phase 落点或触发条件 | IN-07 |

### Defer（记录在账本，后续 phase 处理）

| ID | Deferred Item | Why Defer in v0.1.1 | Trigger to Promote | Dependencies |
|----|---------------|---------------------|--------------------|--------------|
| DF-01 | 完整 JWKS 缓存与轮换策略 | 当前里程碑先补最小可发布安全线 | 多 issuer/多环境签名验证需求明确 | IN-01 稳定后 |
| DF-02 | 细粒度 RBAC/权限模型 | 与“架构收敛”相比收益不成比例 | 出现多角色业务场景与审计需求 | IN-01 + 租户模型 |
| DF-03 | 决策账本自动生成器（从 PR/Issue 抽取） | 非闭环必需，短期可人工维护 | 账本条目规模增长导致维护成本上升 | IN-07 数据结构稳定 |
| DF-04 | 跨语言契约兼容矩阵（版本协商） | v0.1.1 只要求同步闭环，不做多版本协议 | 需要向外部客户端开放稳定 API | IN-04/IN-05 |

### Explicitly Out of Scope（本里程碑明确不做）

| ID | Out-of-Scope Item | Reason | Revisit Window |
|----|-------------------|--------|----------------|
| OOS-01 | 新增认证方式（如 email/password） | 与当前“安全硬化与边界收敛”目标无关，扩大攻击面与实现面 | 认证功能里程碑 |
| OOS-02 | 业务功能扩展（新页面/新模块） | 该里程碑定位是工程闭环，不是功能扩张 | v0.1.2+ 功能里程碑 |
| OOS-03 | 全栈技术栈替换或大规模迁移 | 违反最小改动原则，风险不可控 | 架构重构专项里程碑 |
| OOS-04 | 引入新重型基础设施组件 | 当前已具备可运行脚手架，新增组件不直接服务本里程碑目标 | 明确性能/运维瓶颈出现后 |

## Feature Dependencies

```
[IN-01 JWT最小安全校验]
    └──requires──> [现有鉴权中间件可测试化]

[IN-04 contracts_api单一契约源]
    └──requires──> [契约边界定义]
    └──enables──> [IN-05 typegen门禁]

[IN-05 typegen门禁]
    └──requires──> [Moon/Just任务编排]
    └──enables──> [契约漂移可检测]

[IN-06 runtime边界收敛]
    └──requires──> [当前命令入口清点]
    └──enables──> [后续agent低认知改动]

[IN-07 决策账本]
    └──requires──> [历史建议汇总]
    └──enables──> [IN-08 future phase摘要]
```

### Dependency Notes

- **IN-04 → IN-05 是硬依赖：** 没有单一契约源，typegen 只能生成“某一份真相”，无法防漂移。
- **IN-07 是管理闭环核心：** 没有决策账本，defer/out-of-scope 会在后续 phase 丢失上下文。
- **IN-06 与 IN-01相互增强：** 运行时边界清晰后，鉴权链路更容易统一落点与测试。

## MVP Definition (for this milestone)

### Launch With (v0.1.1 Done)

- [ ] IN-01 JWT 最小安全校验闭环
- [ ] IN-02 敏感配置 fail-fast
- [ ] IN-03 路径可移植性统一
- [ ] IN-04 `contracts_api` 单一契约源
- [ ] IN-05 `typegen` + `verify` 门禁
- [ ] IN-06 `runtime_tauri`/host 边界收敛
- [ ] IN-07 决策账本（implement/defer/out）
- [ ] IN-08 future phase 摘要沉淀

### Add After Validation (v0.1.2+)

- [ ] DF-01 完整 JWKS 缓存/轮换
- [ ] DF-02 细粒度 RBAC
- [ ] DF-03 决策账本自动化生成
- [ ] DF-04 契约版本协商与兼容矩阵

### Future Consideration (v2+)

- [ ] 更高阶安全策略平台化（多租户策略引擎）
- [ ] 多客户端协议治理（超出 Rust/TS 双端）

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| IN-01 JWT 最小安全校验闭环 | HIGH | MEDIUM | P1 |
| IN-02 敏感配置 fail-fast | HIGH | LOW | P1 |
| IN-03 路径可移植性统一 | MEDIUM | LOW | P1 |
| IN-04 契约单一真相源 | HIGH | MEDIUM | P1 |
| IN-05 typegen 门禁 | HIGH | MEDIUM | P1 |
| IN-06 runtime 边界收敛 | HIGH | MEDIUM | P1 |
| IN-07 决策账本 v1 | HIGH | MEDIUM | P1 |
| IN-08 future phase 摘要 | MEDIUM | LOW | P1 |
| DF-01 完整 JWKS | MEDIUM | HIGH | P2 |
| DF-02 细粒度 RBAC | MEDIUM | HIGH | P3 |

**Priority key:**
- P1: 本里程碑必须完成
- P2: 下个里程碑优先候选
- P3: 明确延期

## Sources

- `.planning/PROJECT.md`（v0.1.1 目标与 target features）
- `.planning/REQUIREMENTS.md`（现有约束与 out-of-scope）
- `.planning/ROADMAP.md`（当前 phase 结构与依赖）
- `.planning/phases/06-google-oauth-authentication/06-CONTEXT.md`（鉴权相关历史决策）
- `.planning/phases/07-multi-tenant-data-isolation/07-CONTEXT.md` 与 `07-RESEARCH.md`（`dangerous::insecure_decode` 历史选择）
- `.planning/research/SUMMARY.md`、`.planning/research/ARCHITECTURE.md`（runtime_tauri 边界建议）

---

*Feature research for: Milestone v0.1.1 architecture convergence*
*Researched: 2026-04-01*
