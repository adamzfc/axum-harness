# Project Research Summary

**Project:** Tauri-SvelteKit-Axum Boilerplate（v0.1.1）
**Domain:** 跨端桌面模板工程收敛（安全、契约、运行时边界、流程治理）
**Researched:** 2026-04-01
**Confidence:** HIGH

## Executive Summary

这不是“新增业务功能”里程碑，而是一个 **生产闭环收口里程碑**：把已有可运行模板从“能跑”提升到“可持续迭代、可审计、可自动验证”。四份研究在结论上高度一致：当前最大风险不是技术栈选错，而是关键边界尚未落地（`contracts_api` 和 `runtime_tauri` 仍是 placeholder）、安全链路仍有历史临时方案（`insecure_decode` / 默认占位 secret），以及任务体系还没把目标转成强约束门禁。

推荐路径是“最小改动、高收益优先”的顺序：先补安全基线（JWT 验签、敏感配置 fail-fast、路径可移植），再补契约闭环（`contracts_api` + `ts-rs` + `typegen` 门禁），然后做运行时边界收敛（新能力走 `runtime_tauri`，native host 瘦身），最后把 moon/just 与决策账本绑定为可执行治理。这个顺序同时满足依赖关系与风险最小化，避免在 v0.1.1 触发全仓重构。

核心风险与缓解：最大风险是“看起来完成但实际没闭环”（文档写了、代码跑了，但无 verify 门禁）。缓解策略是把每个里程碑目标映射到可执行命令与可检测信号：伪造 token 必须 401、typegen 后 git diff 必须为空、生产缺关键 secret 必须启动失败、host 入口不再继续堆业务逻辑。这样 roadmap 才不会在下一 phase 回退。

## Key Findings

### Recommended Stack

详见 [STACK.md](./STACK.md)。v0.1.1 的栈结论是“**增量补强，不换底座**”。

**Core technologies（新增/变更）:**
- `openidconnect@4.0.1`：用于 OIDC discovery + JWKS + ID Token 验签，直接替换 payload-only 解码缺口。
- `ts-rs@12.0.1`：从 Rust DTO 自动导出 TS 类型，建立 Rust/TS 契约单一真相源。
- `tauri-specta@2.0.0-rc.24`（可选二阶段）：命令签名导出 TS 绑定，但因 RC 状态不作为 v0.1.1 必做。
- moon/just 现有能力增强：通过 `deps/inputs/outputs` + `dotenv-load` 把 typegen/verify 变成可缓存、可复现任务图。

### Expected Features

详见 [FEATURES.md](./FEATURES.md)。

**Must have（table stakes / v0.1.1 必做）:**
- IN-01 JWT 最小安全校验闭环
- IN-02 敏感配置分级与 fail-fast
- IN-03 路径可移植性统一
- IN-04 `contracts_api` 单一契约源
- IN-05 `typegen` 纳入 `verify` 并防漂移
- IN-06 `runtime_tauri` 与 native host 边界收敛
- IN-07 决策账本 v1（implement/defer/reject）
- IN-08 deferred 的 future phase 摘要沉淀

**Should have（differentiators）:**
- 决策账本可执行化（决策→任务→验证三向映射）
- 契约漂移 CI 门禁（生成后有 diff 即失败）
- 安全基线可观测（输出 machine-readable 检查结果）

**Defer（v0.1.2+）:**
- DF-01 完整 JWKS 缓存与轮换
- DF-02 细粒度 RBAC
- DF-03 决策账本自动生成器
- DF-04 跨语言契约版本协商

**Reject / Out-of-scope（本里程碑明确不做）:**
- OOS-01 新认证方式（email/password）
- OOS-02 新业务页面/模块扩展
- OOS-03 全栈技术栈替换或大规模迁移
- OOS-04 新增重型基础设施
- 以及架构研究明确 reject：v0.1.1 引入 `tauri-plugin-axum` 作为主通信路径

### Architecture Approach

详见 [ARCHITECTURE.md](./ARCHITECTURE.md)。方向不是重构重写，而是在现有仓库上做 **Contract-first + Strangler-style runtime migration**。

**Major components:**
1. `packages/contracts/api` — 外部 DTO/错误模型唯一真相源，驱动 TS 导出。
2. `packages/adapters/hosts/tauri` (`runtime_tauri`) — 命令注册、状态桥接、事件桥。
3. `apps/client/native/src-tauri` — 保留 builder 与平台壳层装配，剥离业务初始化。
4. `servers/api` — HTTP 路由/中间件/状态注入，业务规则继续下沉 usecases。

### Critical Pitfalls

详见 [PITFALLS.md](./PITFALLS.md)。Top 5：

1. **`insecure_decode` 伪造租户风险** — 必须改为验签+claims 校验并做负向测试。
2. **占位 secret/默认弱配置漏入运行时** — 生产环境 fail-fast，且客户端不得回传 secret。
3. **硬编码绝对路径与 `.env` 本机依赖** — 统一 Tauri path API 与环境注入策略。
4. **typegen 半自动导致契约漂移** — `contracts_api` 单源 + CI diff gate。
5. **native host 继续膨胀** — 以 `runtime_tauri` 承接新能力，入口只保留编排。

## Implications for Roadmap

基于四份研究，建议 roadmap 采用 5 个阶段（与依赖严格对齐）：

### Phase 1: Security Baseline Closure
**Rationale:** 风险最高、收益最大，且是后续所有 phase 的前置门槛。  
**Delivers:** JWT 验签替换、secret fail-fast、路径可移植策略。  
**Addresses:** IN-01 / IN-02 / IN-03。  
**Avoids:** Pitfall 1/2/3。

### Phase 2: Contract-First Typegen Closure
**Rationale:** 无契约单源就无法防漂移，后续边界收敛都会返工。  
**Delivers:** `contracts_api` DTO、`ts-rs` 导出、`typegen` 任务与 drift check。  
**Addresses:** IN-04 / IN-05。  
**Avoids:** Pitfall 4。

### Phase 3: Runtime Boundary Convergence
**Rationale:** 在功能不变前提下先收入口，再引入新增能力走新路径。  
**Delivers:** `runtime_tauri` skeleton、host 瘦身第一步、新能力采用新边界。  
**Addresses:** IN-06。  
**Avoids:** Pitfall 5。

### Phase 4: Workflow Guardrails & Verify Unification
**Rationale:** 把前 3 个阶段从“约定”变成“机器强约束”。  
**Delivers:** `fullstack:dev` / `typegen` / `verify` 统一入口与验收映射。  
**Addresses:** IN-05（持续化）+ IN-06（防回流）。  
**Avoids:** Pitfall 6（文档与执行漂移）。

### Phase 5: Decision Ledger Finalization & Forward Map
**Rationale:** 本里程碑目标包含“决策沉淀”，不做会导致 deferred/out-of-scope 信息丢失。  
**Delivers:** implement-now / defer / reject 全量登记、触发条件、后续 phase 映射。  
**Addresses:** IN-07 / IN-08。  
**Avoids:** 决策上下文在下一轮规划中失真。

### Phase Ordering Rationale

- 安全先于契约：先堵高危漏洞，避免“错误能力被更好地类型化”。
- 契约先于边界：先统一协议，边界迁移才不会双向漂移。
- 边界先于流程：先有结构，再把结构做成 verify 门禁。
- 流程先于沉淀：最后固化账本，确保状态与执行一致。

### Implement-now vs Defer vs Reject（显式结论）

| Bucket | Items | 执行结论 |
|---|---|---|
| **Implement-now（v0.1.1）** | IN-01..IN-08 | 全部纳入当前 roadmap，按 Phase 1→5 实施 |
| **Defer（后续 phase）** | DF-01 JWKS 轮换、DF-02 RBAC、DF-03 账本自动化、DF-04 契约版本协商 | 不在本里程碑实现，但必须在账本保留“升级触发条件” |
| **Reject / Out-of-scope（本里程碑）** | OOS-01..OOS-04，及 v0.1.1 引入 tauri-plugin-axum 主路径 | 明确拒绝进入 v0.1.1，防止范围膨胀 |

### Why deferred/rejected still preserved, and where tracked

- **为什么必须保留：** deferred/reject 都是未来决策的输入，不保留会重复争论、重复踩坑。
- **如何保留：** 写入决策账本（建议目录 `.planning/decisions/`，并在 `.planning/ROADMAP.md` phase 处反向链接）。
- **记录字段最低要求：** `Status(implement/defer/reject)`、`Reason`、`Target Phase/Trigger`、`Verification/Exit Criteria`、`Source Link`。
- **执行约束：** `verify` 阶段检查账本条目完整性，确保 deferred/reject 不是“消失的决定”。

### Brief Next-Phase Outlook（防战略上下文丢失）

v0.1.2 的首要候选应是 **DF-01（JWKS 缓存与轮换）+ DF-03（账本自动化）**：前者继续提升安全韧性，后者降低治理维护成本。DF-02（细粒度 RBAC）与 DF-04（契约版本协商）应在出现明确多角色/外部客户端需求后再晋升，避免提前复杂化。

### Research Flags

**需要 `/gsd-research-phase` 的阶段：**
- Phase 1：JWT/OIDC 验签细节与 token 存储分级方案（安全高风险，需设计评审）。
- Phase 3：runtime_tauri 迁移切片策略（避免一次性迁移回归）。
- v0.1.2 候选 DF-01：多 issuer / 多环境 JWKS 缓存轮换策略。

**可按标准模式直接推进的阶段：**
- Phase 2：`ts-rs` contract-first/typegen（文档充分，路径清晰）。
- Phase 4：moon/just 任务编排与 verify 聚合（已有工具能力匹配）。
- Phase 5：决策账本模板化与字段校验（流程问题，不需新技术探索）。

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | 关键新增栈由 Context7 + 官方文档交叉验证；版本和兼容性明确 |
| Features | HIGH | 与 PROJECT.md 里程碑目标高度一致，implement/defer/reject 边界清晰 |
| Architecture | HIGH | 基于仓库实证（placeholder/fat-host）+ 低风险迁移模式，路径可执行 |
| Pitfalls | HIGH | 均有代码证据与可观测信号，且已映射到 phase 验证项 |

**Overall confidence:** HIGH

### Gaps to Address

- `tauri-specta` 仍为 RC：仅作为后续增强，不进入 v0.1.1 必做范围。
- token 高敏存储（Stronghold 分级）的最终策略需在 Phase 1 设计评审定稿。
- 双数据库物理统一不在当前里程碑；仅做契约/端口收敛并保留后续评估票据。

## Sources

### Primary (HIGH confidence)
- [STACK.md](./STACK.md) — `openidconnect`/`ts-rs`/moon/just 增量栈与迁移顺序
- [FEATURES.md](./FEATURES.md) — IN/DF/OOS 明确边界与验收
- [ARCHITECTURE.md](./ARCHITECTURE.md) — contract-first + runtime 收敛策略
- [PITFALLS.md](./PITFALLS.md) — 关键风险、检测信号、phase 映射
- [.planning/PROJECT.md](../PROJECT.md) — 当前里程碑目标与 active requirements

### Secondary (MEDIUM confidence)
- Context7 聚合源：`/ramosbugs/openidconnect-rs`, `/aleph-alpha/ts-rs`, `/moonrepo/moon`, `/casey/just`, `/specta-rs/tauri-specta`

---
*Research completed: 2026-04-01*  
*Ready for roadmap: yes*
