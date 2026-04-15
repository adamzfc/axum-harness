# Agent-First Distributed Harness 10-Phase Iteration Plan

> 目标：以 **完全体分布式要求** 为基线，先建设 **agent harness**，再进入业务开发。
> 范围：本计划优先解决 **架构不变量、目录边界、模型表达能力、agent 约束、参考样例、校验闭环**。
> 原则：**冻结不变量，不冻结实现；先做 harness，再写业务；参考样例与模板同步演进。**

---

## 0. 计划定位

这不是现有 `docs/architecture/iteration-plan.md` 的替代品。

两者定位不同：

| 文档 | 目标 | 适用阶段 |
|---|---|---|
| `docs/architecture/iteration-plan.md` | 现有仓库后端 L2→L3 gate 收口 | 现状治理 |
| `docs/architecture/distributed-harness-iteration-plan.md` | 面向清空重建的 agent-first distributed harness | 新地基建设 |

本计划默认以下前提成立：

1. 现有业务代码和目录可重建，不把迁移成本作为首要约束。
2. 目标不是“先跑起来”，而是“先把 agent 写对的轨道铺好”。
3. 参考样例固定为：`counter`、`tenant`。
4. Agent 采用分工协作模式：`planner`、`platform-ops-agent`、`service-agent`、`server-agent`、`worker-agent`、`contract-agent`、`app-shell-agent`。
5. 完全体分布式的关键语义必须成为 schema、model、template、validator 的一等公民。

---

## 1. 目标状态

完成本计划后，仓库应具备以下能力：

1. Agent 新增业务能力时，必须先回答状态 owner、交互形态、幂等键、一致性等级、恢复策略。
2. Service 的分布式语义由 `services/<name>/model.yaml` 声明，归属 `service-agent`，不与 `platform-ops-agent` 边界冲突。
3. `platform/model/*` 只保留平台级元数据、全局规则、deployable、workflow、topology、resource、environment。
4. 参考样例能覆盖单实体、CAS、事件传播、多租户、workflow、补偿、projection、replay、authz、跨 service 读等关键模式。
5. Agent 模板、技能文档、codemap、validator、golden examples 彼此一致。
6. 后续新增业务时，主要是“填空”而不是“重新发明结构”。

---

## 2. 总体执行顺序

```text
P1  先冻结仓库入口规则与目标边界
  ↓
P2  扩展 schema，让分布式语义可表达
  ↓
P3  重构 platform/model 的职责边界
  ↓
P4  建立 service-local semantic model
  ↓
P5  重写 codemap / routing / anti-pattern / decision tree
  ↓
P6  精简模块目录并建立参考样例骨架
  ↓
P7  落地参考 service / contracts / server skeleton
  ↓
P8  落地参考 worker / workflow / projection / replay skeleton
  ↓
P9  落地 validator / gates / strict policy
  ↓
P10 落地 golden examples / handoff rules / 开发准入标准
```

硬性规则：

1. P2 未完成前，不允许写新的 reference module 业务实现。
2. P4 未完成前，不允许把 service 语义继续塞进 `platform/model/state/**` 分散文件。
3. P5 未完成前，不允许让 agent 大规模并行开发 reference modules。
4. P8 未完成前，不允许宣称已具备分布式闭环能力。
5. P9 未完成前，不允许把“规则已经写在文档里”视作完成。

---

## 3. 10 个 Phase 详细计划

### Phase 1：冻结入口规则与目标边界

**目标**：先明确这次重建到底在建设什么，避免不同 agent 按不同理解各自发挥。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P1.1 | 更新 `docs/architecture/repo-layout.md`，明确“agent-first distributed harness”目标 | planner | 仓库入口规则文档 | 仓库入口规则与本计划一致 |
| P1.2 | 新建 `docs/architecture/distributed-harness-iteration-plan.md` | planner | 本文档 | 10 phase 与依赖关系清晰 |
| P1.3 | 新建 `platform/model/README.md`，定义平台模型职责边界 | platform-ops-agent | 模型层说明 | 能解释 model 与 services/** 的职责切分 |
| P1.4 | 更新 `AGENTS.md` 或相关总控文档中的“完全体分布式优先”原则 | planner | 总控协议对齐 | 新 agent 读入口文档后不会误判为“单点优先” |

**关键决策**：

1. Service 的业务语义声明放在 `services/<name>/model.yaml`。
2. `platform/model/*` 只保留平台级元数据与全局规则。
3. 参考样例固定为 `counter`、`tenant`、`settings`、`admin`。

**完成定义**：

1. 新成员只读 `repo-layout.md`、`platform/model/README.md`、本计划，就能理解为什么是 harness-first。
2. 文档中不再混淆“平台模型”和“service 语义声明”的归属。

---

### Phase 2：扩展 Schema，让分布式语义可表达

**目标**：把“应该声明什么”从经验变成 schema 字段。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P2.1 | 扩展 `platform/schema/service.schema.json` | platform-ops-agent | 新 service schema | 支持 `owns_entities`、`accepted_commands`、`published_events`、`served_queries`、`cross_service_reads`、`spec_completeness` |
| P2.2 | 扩展 `platform/schema/workflow.schema.json` | platform-ops-agent | 新 workflow schema | 支持 step 级 `idempotency_key`、`checkpoint`、`max_retries`、顶层 `compensation`、`recovery` |
| P2.3 | 扩展 `platform/schema/deployable.schema.json` | platform-ops-agent | 新 deployable schema | 支持 `statefulness`、`required_identity`、`required_storage`、`failure_domain`、`scaling_axis` |
| P2.4 | 扩展 `platform/schema/policy.schema.json` | platform-ops-agent | 新 policy schema | 新增 `consistency`、`partitioning`、`schema-evolution` 类型 |
| P2.5 | 审核 schema 是否与 4 个 reference service 的需求吻合 | planner + platform-ops-agent | schema review note | 参考 service 语义都可被表达，无需临时加字段 |

**关键字段最少集合**：

1. `owner service`
2. `write_authority`
3. `idempotency_key`
4. `consistency_level`
5. `ordering_scope`
6. `replayable`
7. `checkpoint_policy`
8. `compensation`
9. `statefulness`
10. `required_identity`

**完成定义**：

1. 所有分布式关键语义都能在 schema 中表达。
2. 新建一个 service / workflow / deployable 时，不需要再靠 README 补缺字段。

---

### Phase 3：重构 `platform/model/*` 的职责边界

**目标**：让平台模型只做平台的事，不吞并 service 语义。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P3.1 | 重新定义 `platform/model/services/*.yaml` 的最小职责 | platform-ops-agent | 平台级 service model | 仅包含平台元数据，如 criticality、tenant scope、logical deps、status |
| P3.2 | 新建 `platform/model/state/ownership-map.yaml` | platform-ops-agent | 全局 owner map | 所有核心实体与 owner service 一一映射 |
| P3.3 | 新建 `platform/model/state/consistency-defaults.yaml` | platform-ops-agent | 全局 consistency 默认规则 | 默认 eventual / read-your-write 规则可追踪 |
| P3.4 | 新建 `platform/model/state/idempotency-defaults.yaml` | platform-ops-agent | 全局 idempotency 默认规则 | 默认 dedupe window 和 scope 可追踪 |
| P3.5 | 新建 `platform/model/partitioning/defaults.yaml` | platform-ops-agent | 全局 partition 默认规则 | tenant / partition key 默认策略清晰 |
| P3.6 | 新建 `platform/model/failures/domains.yaml` 与 `platform/model/slo/defaults.yaml` | platform-ops-agent | 全局 failure / SLO baseline | 可以描述 failure domain 和基本预算 |

**目录原则**：

1. `platform/model/*` 不存放每个 service 的细粒度业务语义。
2. `platform/model/state/*` 只放全局规则，不放每个 service 各自一堆零散 YAML。
3. per-service semantic model 归属 `services/<name>/model.yaml`。

**完成定义**：

1. `platform/model/` 读起来像平台控制面，不像业务实现散落区。
2. `platform-ops-agent` 不需要理解每个 service 的内部业务细节也能维护 model。

---

### Phase 4：建立 Service-Local Semantic Model

**目标**：让 service-agent 在自己的边界内声明完整分布式语义。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P4.1 | 为每个 reference service 设计 `services/<name>/model.yaml` 结构 | service-agent | semantic model 模板 | `counter/tenant` 都能完整表达 |
| P4.2 | 在 `counter-service/model.yaml` 中声明单实体、CAS、event 发布 | service-agent | counter semantic model | 可覆盖最小完整链路 |
| P4.3 | 在 `tenant-service/model.yaml` 中声明多实体、多租户、workflow 入口 | service-agent | tenant semantic model | onboarding / invite 的前提语义完整 |
| P4.6 | 定义 `spec_completeness` 分级 | service-agent + planner | completeness 规则 | 区分 `declared` / `verified` / `tested` |

**每个 `model.yaml` 至少必须回答**：

1. 它拥有哪个实体。
2. 它接受哪些 command。
3. 它发布哪些 event。
4. 它提供哪些 query。
5. 它要求什么一致性。
6. 它如何处理跨 service 读取。
7. 它当前语义声明完成到什么级别。

**完成定义**：

1. service-agent 无需改 `platform/model/**` 也能完整表达自身语义。
2. 任意一个 reference service 的完整语义能在单个目录中被理解。

---

### Phase 5：重写 Codemap / Routing / Anti-Patterns / Decision Tree

**目标**：把人类架构经验转成 agent 可执行规则。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P5.1 | 重写 `agent/codemap.yml` 的 principles、layers、write_boundaries | planner | 新 codemap | 与新目录结构一致 |
| P5.2 | 新增 `state_rules` | planner | owner / consistency / projection / idempotency 规则 | agent 能据此判断语义缺口 |
| P5.3 | 新增 `messaging_rules` | planner | 至少一次、乱序、重复、重试假设 | agent 不会默认 exactly-once |
| P5.4 | 新增 `workflow_rules` | planner | 何时必须 workflow 化 | handler 长事务可被拒绝 |
| P5.5 | 新增 `anti_patterns` | planner | machine-readable 反模式列表 | validator 后续可消费 |
| P5.6 | 新增 `decision_tree` 与 `change_playbooks` | planner | add-service / add-worker / change-contract 等流程 | agent 新建模块时有固定问答顺序 |
| P5.7 | 调整 `routing-rules.yml` 与 `subagents.yml` | planner | 路由与边界对齐 | service-agent 可合法维护 `services/*/model.yaml` |

**关键要求**：

1. codemap 不是目录清单，而是架构裁判器。
2. 禁止只写原则不写决策树。
3. write boundary 必须和真实 agent 分工一致。

**完成定义**：

1. 新 agent 只读 `codemap.yml`，就知道何时声明 owner、何时走 workflow、何时拒绝 cross-service write。
2. `codemap.yml` 可作为后续 validator 的规则来源，而不是重复定义第二套逻辑。

---

### Phase 6：精简目录并建立 Reference Skeleton

**目标**：清理冗余模块，把参考样例的地基搭出来。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P6.1 | 将 reference service 收敛为 `counter`、`tenant` 两个主样例 | planner + service-agent | 精简后的服务矩阵 | 两个样例覆盖第一批核心模式 |
| P6.2 | 对 `auth`、`chat`、`agent`、`indexing` 等降级为 stub 或 planned | planner | 目录状态收敛 | 不再误导 agent 以为这些是立即实现对象 |
| P6.3 | 移除或改造 `services/event-bus`，把基础设施性质职责回归 `packages/messaging/` 与 `workers/outbox-relay/` | planner + platform-ops-agent | 目录职责清晰 | event-bus 不再伪装成业务 service |
| P6.4 | 建立 4 个 reference service 的 skeleton 目录 | service-agent | 4 个 service 骨架 | 目录结构与 templates 对齐 |
| P6.5 | 建立参考 server / worker / workflow 的占位骨架 | server-agent + worker-agent + platform-ops-agent | 最小闭环骨架 | 可作为模板验证基线 |

**精简原则**：

1. reference module 的数量以覆盖模式为准，不以覆盖未来业务名词为准。
2. 能通过 `packages/*` 表达的能力，不要伪装成 service。

**完成定义**：

1. 仓库里只保留真正承担参考职责的样例。
2. 未来 agent 不会因为一堆 planned 目录误判当前优先级。

---

### Phase 7：落地 Reference Service / Contract / Server Skeleton

**目标**：用 reference service 验证 schema 和 codemap 不是纸上谈兵。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P7.1 | 为 `counter` 定义 contracts、service library、最小 server endpoint | contract-agent + service-agent + server-agent | 最小完整同步链路 | 可完成 command → state change → query |
| P7.2 | 为 `tenant` 定义 create/invite 相关 contracts 与 service skeleton | contract-agent + service-agent | 多实体与 workflow 前置语义 | invite / onboarding 的接口边界明确 |
| P7.3 | 收敛非参考 service 到 `stub` / `deprecated` 状态，并补全 `model.yaml` | planner + service-agent | 清晰的目录分类 | agent 不会把 planned/deprecated 目录误判为活跃参考 |
| P7.4 | 统一 `counter` / `tenant` README、`model.yaml`、`src/lib.rs` 的 reference skeleton 口径 | service-agent | reference skeleton 对齐 | reference 模块与模板描述一致 |
| P7.5 | 统一 service README 与 model.yaml 结构 | service-agent | 参考服务文档 | 每个 service 都能作为 agent 学习样例 |

**本 phase 不要求**：

1. 写完整 UI。
2. 写完整 infra。
3. 覆盖所有 planned service。

**完成定义**：

1. 两个 reference service 都有可读的 `model.yaml`、README、contracts、基本目录骨架。
2. 至少 `counter` 拥有最小可运行同步链路。

---

### Phase 8：落地 Reference Worker / Workflow / Projection / Replay Skeleton

**目标**：补齐异步与恢复半边天，避免“只有同步入口样例”。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P8.1 | 建立 `outbox-relay` 参考 worker 骨架 | worker-agent | relay skeleton | 明确 idempotency、retry、checkpoint |
| P8.2 | 建立 `projector` 参考 worker 骨架 | worker-agent | projector skeleton | 明确 replay、rebuild、lag SLO |
| P8.3 | 重写 `tenant-onboarding` workflow model | platform-ops-agent + service-agent | workflow reference | 含 step-level idempotency、checkpoint、compensation |
| P8.4 | 重写 `invite-member` workflow model | platform-ops-agent + service-agent | workflow reference | 长事务补偿与人工介入点清晰 |
| P8.5 | 为 counter 增加 projection / replay 样例 | worker-agent + service-agent | projection reference | 可解释可删可重建 |
| P8.6 | 为 worker README 强制写明 strategy requirements | worker-agent | 可审查策略文档 | 每个 worker 都说明 idempotency / retry / replay |

**关键要求**：

1. 任何 workflow 样例都必须显式声明 compensation。
2. 任何 worker 样例都必须说明 checkpoint 与恢复顺序。
3. 任何 projection 样例都必须说明可重建性。

**完成定义**：

1. reference harness 不再只有同步半边，而具备分布式状态推进的最小闭环。
2. 新 worker / workflow 可以直接按样例填空。

---

### Phase 9：落地 Validator / Gates / Strict Policy

**目标**：把规则从“文档存在”升级成“违规会被拦截”。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P9.1 | 实现 model schema validator | platform-ops-agent | schema 校验器 | 所有 model 文件都能校验 |
| P9.2 | 实现 owner uniqueness validator | platform-ops-agent | owner 校验器 | 一个核心实体不能出现多个 owner |
| P9.3 | 实现 workflow completeness validator | platform-ops-agent | workflow 校验器 | 缺 compensation / checkpoint 会失败 |
| P9.4 | 实现 event metadata completeness validator | contract-agent + platform-ops-agent | event 校验器 | ordering / replay / compatibility 缺失会失败 |
| P9.5 | 实现 projection rebuildability validator | worker-agent + platform-ops-agent | projection 校验器 | projection 未声明 rebuildable 会失败 |
| P9.6 | 实现 topology boundary validator | platform-ops-agent | topology 校验器 | topology 不允许改变状态语义 |
| P9.7 | 接入 `just validate-platform`、`just validate-state`、`just validate-workflows`、`just verify-replay` | platform-ops-agent | 统一 gate 入口 | 本地与 CI 均可跑 |
| P9.8 | 将核心规则切为 strict | planner + platform-ops-agent | strict gate policy | owner 缺失、workflow 缺失 recovery、generated 手改会阻塞 |

**最先进入 strict 的规则**：

1. owner 缺失
2. cross-service direct write
3. workflow 缺少 compensation / checkpoint
4. projection 未声明 rebuildable
5. generated 目录手改

**完成定义**：

1. 不是“知道应该这样做”，而是“不这样做就过不了 gate”。
2. Agent 不能自由发挥越过分布式边界。

---

### Phase 10：Golden Examples / Handoff Rules / 开发准入标准

**目标**：形成真正可复制的 agent 开发操作系统，而不是一次性改造工程。

**主要任务**：

| 步骤 | 任务 | 分配 agent | 产出 | 验收 |
|---|---|---|---|---|
| P10.1 | 建立 `verification/golden/` 最小完整样例 | planner + 各领域 agent | golden example 套件 | 覆盖 command / event / query / workflow / projection / replay / authz |
| P10.2 | 建立 `agent/templates/*`，与 reference modules 对齐 | planner + 各领域 agent | 模板集 | 模板与 reference 不漂移 |
| P10.3 | 更新 7 个 agent skill 文档 | planner | 新版 skill 协议 | 每个 agent 都知道新字段和新规则 |
| P10.4 | 建立 handoff 协议：service / worker / platform-ops 之间的交接要求 | planner | handoff checklist | 多 agent 并行开发不相互踩边界 |
| P10.5 | 明确“允许进入大规模业务开发”的准入标准 | planner | entry criteria | 达标前不得大规模并行造业务 |

**大规模业务开发准入标准**：

1. `counter` 和 `tenant` 两个 reference service 的 semantic model 稳定。
2. 至少 1 个同步链路、1 个异步链路、1 个 workflow、1 个 replay 样例跑通。
3. codemap、templates、skills、validators 对齐。
4. 新 agent 加入后，不需要额外口头解释即可按 reference 填空。

**完成定义**：

1. harness 成为长期操作系统，而不是一次性文档。
2. 新业务开发可以安全并行化。

---

## 4. Phase 与 Agent 的映射

| Phase | planner | platform-ops-agent | service-agent | server-agent | worker-agent | contract-agent | app-shell-agent |
|---|---|---|---|---|---|---|---|
| P1 | 主导 | 支持 | - | - | - | - | - |
| P2 | 审核 | 主导 | - | - | - | - | - |
| P3 | 支持 | 主导 | - | - | - | - | - |
| P4 | 审核 | - | 主导 | - | - | - | - |
| P5 | 主导 | 支持 | 支持 | 支持 | 支持 | 支持 | - |
| P6 | 主导 | 支持 | 主导 | 支持 | 支持 | - | - |
| P7 | 协调 | - | 主导 | 支持 | - | 主导 | - |
| P8 | 协调 | 主导 workflow | 支持 | - | 主导 worker | - | - |
| P9 | 审核 strict 策略 | 主导 | 支持 | 支持 | 支持 | 支持 | - |
| P10 | 主导 | 支持 | 支持 | 支持 | 支持 | 支持 | 视前端范围决定 |

说明：

1. `app-shell-agent` 本轮不是主角，但不排除后续为 SDK 消费样例补最小前端壳层。
2. `planner` 负责跨 phase 收敛，不负责落业务实现。

---

## 5. 风险与禁止事项

### 5.1 本计划最大的风险

| 风险 | 描述 | 缓解方式 |
|---|---|---|
| 目录过度拆分 | 为了表达语义而制造 200+ 零散 YAML | 采用 service-local semantic model，平台层只保留全局规则 |
| agent 边界冲突 | 语义文件放在错误目录，导致谁都写不好 | 以 agent writable boundary 反推文件归属 |
| 参考样例失真 | template 和 reference 漂移 | reference 与 template 同步开发 |
| 文档强、校验弱 | 规则只写在文档里无法阻止违规 | Phase 9 必须进入 strict gate |
| 提前并行造业务 | harness 未稳就让多个 agent 大量写业务 | P10 前禁止大规模业务并行开发 |

### 5.2 明确禁止

1. 禁止在 P2 之前新写 reference service 业务代码。
2. 禁止在 P4 之后继续把 service 语义拆散到 `platform/model/state/**` 中。
3. 禁止把 `event-bus` 这类基础设施能力继续伪装成业务 service。
4. 禁止在没有 workflow model 的前提下写跨 service 长事务 handler。
5. 禁止在没有 replay / rebuild 声明的情况下把 projection 当作真相源。

---

## 6. 完成判定

只有当以下问题都能被 model、template、validator、reference 共同回答时，本计划才算完成：

1. 这个新实体归谁拥有？
2. 这个变更是 command、event、query 还是 workflow？
3. 这个副作用的幂等键是什么？
4. 这个 query 的一致性等级是什么？
5. 这个 worker 如何 checkpoint 与恢复？
6. 这个 projection 是否可删可重建？
7. 这个 deployable 是否需要 stable identity 与 persistent storage？
8. topology 切换后，状态语义是否不变？
9. agent 是否无需额外口头解释就能按 reference 填空？
10. 违反上述规则时，gate 是否会阻断？

若这 10 个问题仍主要依赖人脑解释而不是 harness 自动约束，则本计划仍未完成。

---

## 7. 下一步执行建议

推荐启动顺序：

1. 先做 P1 + P2：把入口规则和 schema 钉死。
2. 再做 P3 + P4：把平台层和 service 层的语义归属切干净。
3. 再做 P5：重写 codemap，让 agent 真正有操作手册。
4. 再做 P6 + P7 + P8：用 reference modules 验证 harness 真能落地。
5. 最后做 P9 + P10：把规则编译化，建立长期可复制的开发系统。

一句话原则：

> 先把 agent 写对的轨道铺好，再让 agent 跑快；不要先让 agent 跑快，再靠人工纠偏。
