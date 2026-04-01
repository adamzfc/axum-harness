# Architecture Research

**Domain:** Tauri 2 + SvelteKit + Axum boilerplate（milestone v0.1.1 架构收敛）  
**Researched:** 2026-04-01  
**Confidence:** HIGH（仓库现状 + 官方文档交叉验证）

## Standard Architecture（基于当前仓库收敛，不推翻重来）

### System Overview

```text
┌───────────────────────────────────────────────────────────────────────┐
│ apps/client/web (SvelteKit UI)                                       │
│  - invoke Tauri commands / call runtime_server HTTP                  │
└──────────────────────────────┬────────────────────────────────────────┘
                               │
                               │ IPC (primary for native capabilities)
                               │
┌──────────────────────────────▼────────────────────────────────────────┐
│ apps/client/native/src-tauri (native host, currently fat)            │
│  - plugin registration, setup, commands, sync bootstrap              │
└──────────────────────────────┬────────────────────────────────────────┘
                               │
                               │ should delegate to
                               │
┌──────────────────────────────▼────────────────────────────────────────┐
│ packages/adapters/hosts/tauri (runtime_tauri, currently placeholder) │
│  - Tauri runtime adapter boundary                                     │
│  - command handlers + state wiring + event bridge                    │
└──────────────────────────────┬────────────────────────────────────────┘
                               │
          ┌────────────────────┴────────────────────┐
          │                                         │
┌─────────▼─────────┐                    ┌──────────▼──────────┐
│ packages/core/*   │                    │ servers/api          │
│ domain + usecases │                    │ runtime_server (Axum)│
│ business + ports  │                    │ HTTP routes/mw/state │
└─────────┬─────────┘                    └──────────┬──────────┘
          │                                         │
          └────────────────────┬────────────────────┘
                               │
┌──────────────────────────────▼────────────────────────────────────────┐
│ packages/contracts/api (contracts_api, currently placeholder)         │
│  - shared request/response/error DTO                                  │
│  - Rust->TS typegen single source of truth                            │
└───────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities（收敛后）

| Component | Owns | Must NOT own |
|---|---|---|
| `packages/core/domain` | 纯业务概念、端口 trait、新类型（TenantId） | Axum/Tauri/libsql/surreal 细节 |
| `packages/core/usecases` | 用例编排、业务规则、跨 port 协调 | HTTP/IPC 协议、插件初始化 |
| `packages/contracts/api` | 外部契约 DTO、错误码枚举、TS 导出标注 | 数据库访问、业务流程 |
| `servers/api` | HTTP 路由、中间件、OpenAPI、运行时 state | 领域规则本体 |
| `packages/adapters/hosts/tauri` (`runtime_tauri`) | Tauri command + state + event bridge | 具体窗口/托盘最终装配（可由 host 进行最薄组装） |
| `apps/client/native/src-tauri` (native host) | 最终 app builder、插件装配、平台入口 | 大量业务逻辑、sync/auth 细节实现 |

## Current Drift（基于已读代码）

1. **runtime_tauri 与 contracts_api 都是 placeholder**，但 Cargo/moon 已纳入工作区。  
2. **native host 过胖**：`src-tauri/src/lib.rs` 集中 setup、auth/sync bootstrap、tray、panic hook、env 路径逻辑。  
3. **contracts 未落地**：`routes/tenant.rs` 与 Tauri commands 自己定义序列化结构，易漂移。  
4. **数据库策略存在分叉风险**：
   - server: SurrealDB（+可选 Turso）
   - native: Embedded libsql（+sync engine）
   当前“并行可运行”，但尚未形成契约层的一致抽象与迁移边界。

---

## Integration Points（精确到“改哪里/加哪里”）

### 1) 新 capability 接入主路径（推荐）

**Flow:** UI → Tauri command（runtime_tauri）→ usecases → domain ports → adapter(impl) → data

#### Modify vs Add

- **Add（v0.1.1 必做）**
  - `packages/contracts/api/src/lib.rs`：定义外部 DTO（如 `TenantInitRequest/Response`、`ApiError`）。
  - `packages/adapters/hosts/tauri/src/`：新增 `commands/*`, `state.rs`, `bridge.rs`。
  - typegen 任务（moon/just）输出到 web client types 目录。
- **Modify（v0.1.1 必做）**
  - `apps/client/native/src-tauri/src/lib.rs`：仅保留 builder+插件注册+调用 runtime_tauri init。
  - `apps/client/native/src-tauri/src/commands/*`：逐步搬迁到 `runtime_tauri`（先新增 capability 走新路径，旧命令后续迁）。
  - `servers/api/src/routes/*`：入参/出参改用 `contracts_api` DTO，避免重复定义。
- **Defer（后续）**
  - 旧命令一次性全量迁移（风险高，不符合最小改动）。

### 2) runtime_server 新 capability 接入点

**现有基线可复用：** `servers/api/src/lib.rs` 已有 router + middleware layering，`state.rs` 已可持有 db/cache/http_client/config。  
**接入点：**

- Route module：`servers/api/src/routes/<feature>.rs`
- 注册：`servers/api/src/routes/mod.rs::api_router()`
- 中间件：默认走 `tenant_middleware`（路由层已挂）
- DTO：从 `contracts_api` 引入

### 3) contracts/typegen 数据流（必须闭环）

```text
contracts_api (Rust DTO + TS derive/export)
        │
        ├─ cargo task: generate TS declarations
        │
        └─ output -> apps/client/web/.../types/generated.ts
                       │
                       └─ frontend invoke/fetch wrappers consume same types
```

**建议实现（v0.1.1）：**
- `contracts_api` 引入 `serde` + `ts-rs`（Context7 已验证）
- DTO 使用 `#[derive(Serialize, Deserialize, TS)]` + `#[ts(export)]`
- 增加 `moon` task: `contracts-api:typegen`（可通过 `cargo test -p contracts_api` 触发导出，或单独 binary）
- 根任务补齐：`typegen`, `verify`，并让 `check-all/test-all` 依赖 typegen 完整性检查

### 4) Boundary Ownership（硬边界）

1. **contracts_api = 协议所有权**：谁暴露给 UI/HTTP，谁必须先落 contracts。  
2. **usecases = 行为所有权**：业务流程只在 usecases；route/command 仅组装输入输出。  
3. **runtime_tauri/runtime_server = 传输与运行时所有权**：HTTP/IPC、middleware、plugin lifecycle。  
4. **native host = 壳层所有权**：平台入口、最终装配，不持有业务复杂度。

---

## v0.1.1: Implement vs Defer（质量闸门要求）

## 必须在 v0.1.1 实施

1. **contracts_api 最小可用落地 + typegen 闭环**
   - Why: 当前最大漂移源就是“契约未收敛”。
   - Risk if skip: Rust/TS 类型持续分叉，后续每个 feature 返工成本指数上升。

2. **runtime_tauri 起壳并承接“新增 capability”**
   - Why: 不要求一次性迁完旧命令，但必须建立“新能力走新边界”的增量规则。
   - Risk if skip: native host 继续膨胀，后续迁移风险更高。

3. **native host 瘦身第一步（提取 wiring）**
   - 至少把 command 注册与部分 setup 抽到 runtime_tauri 暴露函数。
   - 保持功能不变，降低回归风险。

4. **Moon/Just 任务补全：`typegen`、`verify`、`fullstack:dev`**
   - Why: 体系化保证架构收敛可持续，不靠人工记忆。

## 明确 defer 到后续 phase

1. **tauri-plugin-axum 全面接入（defer）**
   - 结论：**v0.1.1 不引入**。
   - 依据：该插件存在且可用（docs.rs/crates 有最新版本），但文档覆盖低（约 12.5%）、协议差异（Windows/Android 与 macOS/Linux URL 方案不同）、流式能力存在限制说明；对当前里程碑“最小改动收敛”目标不友好。
   - 风险：提前接入会把 IPC/HTTP 边界再复杂化，增加调试面。
   - 后续何时引入：当且仅当需要 **WebView 内以 fetch/axios 方式复用 Axum Router** 且有明确收益（例如大量现成 HTTP 客户端中间件复用）。

2. **旧 auth/sync 命令全量迁入 runtime_tauri（defer）**
   - 先让新增能力走新路径，旧功能分批迁移。

3. **数据库单一化重构（defer）**
   - v0.1.1 做“策略收敛与接口收敛”，不做“物理栈统一重写”。

---

## DB Convergence Strategy（下游关心点）

### 结论

**v0.1.1 采用“契约收敛 + 端口收敛”，不做 Surreal/libsql 二选一重构。**

### 理由

- 当前双栈已在代码中成形：
  - server 偏 Surreal（且有 Turso 分支）
  - native 偏 libsql（本地优先 + sync）
- 里程碑目标是架构收敛与最小改动，不是基础设施重建。

### 执行方式

1. **对外统一 contracts DTO**（无论底层 Surreal 或 libsql）。
2. **usecases 层只依赖 port trait**，运行时选择具体实现。
3. **在 runtime_server state 中明确 provider 开关语义**（已有 `db_provider`，继续强化）。
4. **新增 capability 优先选“一个主路径”**（建议 server Surreal or Turso 二选一，不在同一功能同时支持两种写路径）。

### 风险提示

- 若继续双写/双实现但无统一契约：维护成本极高（HIGH risk）。
- 若 v0.1.1 强行统一到底层单 DB：改动过大、回归面过广（HIGH risk）。

---

## Recommended Build Order（低风险迁移顺序）

1. **Phase A — Contracts First（低风险高收益）**
   - 建 `contracts_api` DTO + error model
   - 接入 ts-rs typegen
   - web/client 消费 generated types

2. **Phase B — Runtime Tauri Skeletonize**
   - `runtime_tauri` 增加 `init_commands()` / `init_state()`
   - native host 改为调用这些 init（功能保持不变）

3. **Phase C — New Capability on New Path**
   - 选一个小 capability（如 tenant init config read）从 UI→runtime_tauri→usecases→contracts 走通
   - 验证新边界可用

4. **Phase D — Server DTO Convergence**
   - `routes/tenant.rs` 等使用 contracts DTO 替换本地定义
   - OpenAPI 保持可用（utoipa 可继续）

5. **Phase E — Tasking/Guardrails**
   - moon/just 增加 `fullstack:dev`、`typegen`、`verify`
   - CI 加“typegen drift check”（生成后无 diff）

6. **Phase F — Deferred Backlog Ticketing（非实现）**
   - `tauri-plugin-axum` 评估 spike
   - legacy command 分批迁移计划
   - DB 物理统一可行性评估

---

## Architectural Patterns（本里程碑建议采用）

### Pattern 1: Strangler-Style Runtime Migration
**What:** 新能力走 `runtime_tauri`，旧逻辑保留在 native host，逐步替换。  
**When:** 现有 host 已过胖且要最小改动。  
**Trade-off:** 短期双路径并存，但总体风险最低。

### Pattern 2: Contract-First External Boundary
**What:** 先定义 contracts，再写 route/command。  
**When:** 需要 Rust/TS 同步、减少漂移。  
**Trade-off:** 初期多一步定义，但后期减少反复改接口。

### Pattern 3: Runtime Adapter Thin Layer
**What:** route/command 只做协议转换、鉴权上下文注入、调用 usecases。  
**When:** 所有新增 API/IPC。  
**Trade-off:** 需要更严格代码评审，防止逻辑回流到适配层。

---

## Anti-Patterns to Avoid（针对当前仓库）

1. **在 native host 继续新增复杂业务**  
   - Instead: 新逻辑只进 `runtime_tauri`。

2. **路由/命令自定义本地 DTO，不进 contracts**  
   - Instead: contracts 先行，route/command 引用。

3. **为引入 tauri-plugin-axum 立即改通信主路径**  
   - Instead: 保持现有 IPC + Axum 并行职责；插件单独 spike。

4. **v0.1.1 强制单 DB 重构**  
   - Instead: 先接口/契约统一，再评估物理统一。

---

## Integration Points Summary（可直接给 roadmap）

| Topic | v0.1.1 | Modify | Add | Defer |
|---|---|---|---|---|
| `runtime_tauri` 落地 | ✅ | native host builder wiring | runtime_tauri commands/state modules | 全量旧命令迁移 |
| `contracts_api` + typegen | ✅ | server routes / frontend wrappers 引用 contracts | DTO + TS export + moon/just task | 全域 DTO 一次性替换 |
| tauri-plugin-axum | ❌ | - | - | ✅ 后续评估 spike |
| DB convergence | ✅（接口层） | 明确 provider 语义与 usecase 依赖 | contracts 统一输出 | 物理层单栈重构 |

---

## Sources

- Repository evidence（HIGH）
  - `apps/client/native/src-tauri/src/lib.rs`（host 过胖、硬编码路径、setup 集中）
  - `packages/adapters/hosts/tauri/src/lib.rs`（placeholder）
  - `packages/contracts/api/src/lib.rs`（placeholder）
  - `servers/api/src/lib.rs`, `routes/tenant.rs`, `middleware/tenant.rs`, `state.rs`
  - `Cargo.toml`, `moon.yml`, `Justfile`

- Context7 / Official docs（HIGH）
  - Tauri docs（plugin command exposure / init patterns）
  - Axum 0.8 middleware layering（ServiceBuilder / route_layer patterns）
  - ts-rs docs（`#[derive(TS)]`, `#[ts(export)]`, export dir behavior）
  - utoipa + axum integration patterns（SwaggerUi merge）

- Official crate docs（MEDIUM）
  - `tauri-plugin-axum` docs.rs/crates（存在且可用，但文档覆盖较低，能力/限制需谨慎评估）

---
*Architecture research for milestone v0.1.1 convergence*  
*Focus: integration points, boundary ownership, contracts/typegen flow, low-risk migration order*
