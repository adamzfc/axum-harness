# Stack Research

**Domain:** Tauri-SvelteKit-Axum 模板（v0.1.1 增量栈研究，仅覆盖新增能力）
**Researched:** 2026-04-01
**Confidence:** HIGH（核心结论已用 Context7/官方文档校验）

## Recommended Stack (Only Additions/Changes for v0.1.1)

> 基线已存在：Tauri 2.10.3 / Axum 0.8.8 / SvelteKit 2.55 / moon / just。以下仅列“新增或需要替换”的栈。

### Core Technologies (新增/变更)

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| `openidconnect` (Rust) | `4.0.1` | Google OIDC 发现文档 + JWKS 拉取 + ID Token 校验 | 直接补上当前 `id_token` “只解码不验签”缺口；比手写 JWKS 轮询更稳，最小重构即可接入现有 auth 流程。 |
| `ts-rs` (Rust) | `12.0.1` | 从 `contracts_api` Rust DTO 生成 TS 类型 | 稳定版、无运行时开销、与 `serde` 生态契合；适合“先类型闭环、后命令闭环”的低风险迁移。 |
| `tauri-specta` (Rust, 可选二阶段) | `2.0.0-rc.24` | Tauri command 签名到 TS 绑定的自动导出 | 能把 `invoke` 字符串协议升级成类型安全协议；但当前是 rc，建议作为 v0.1.1 后半段可选落地。 |

### Supporting Libraries / Process Tools

| Library/Tool | Version | Purpose | When to Use |
|--------------|---------|---------|-------------|
| `jsonwebtoken` | `10.3.0`（已在 workspace） | JWT claims/算法校验基线 | 保留用于服务端 access token 验证；OIDC id_token 验签优先交给 `openidconnect` 流程。 |
| moon task options (`deps`/`inputs`/`outputs`/`runFromWorkspaceRoot`) | 当前 moon 版本 | 构建 typegen + verify DAG | 将“验证链”变成可缓存、可组合任务，而不是散落脚本。 |
| just settings (`set dotenv-load`, recipe deps) | just `1.43.x` 文档语义 | 本地开发统一入口 | 补齐 `.env` 自动加载、全栈 verify 聚合入口，保持新同学低心智负担。 |
| ADR 文档模板（无新依赖） | N/A | 决策账本（decision ledger） | 用纯 Markdown 模板 + Moon/Just 校验即可，不引入 docs 平台依赖。 |

## Capability-by-Capability Stack Changes

### 1) JWT verification hardening

**当前现状（代码证据）**
- `apps/client/native/src-tauri/src/commands/auth.rs` 第 200 行注释明确写了“signature verification deferred to v2”。

**推荐增量**
1. 引入 `openidconnect = "4.0.1"` 到 `apps/client/native/src-tauri/Cargo.toml`（或 workspace 统一管理）。
2. 在 `handle_oauth_callback` 里，token exchange 后不再只做 base64 payload decode；改为：
   - 通过 issuer discovery 获取 provider metadata（含 `jwks_uri`）
   - 用 `id_token_verifier` 校验签名 + `iss` + `aud` + `nonce`
   - 保留对 `exp`/`nbf` 容忍窗口策略（leeway）
3. 保留现有 `jsonwebtoken`，用于非 OIDC token 场景（如服务端自签 access token）

**最小改动集成点**
- 文件：`apps/client/native/src-tauri/src/commands/auth.rs`
- 仅替换“第 200~209 行解码逻辑段”，其余 store/refresh 机制可先不动。

---

### 2) Rust↔TS contract/type generation pipeline

**当前现状**
- `packages/contracts/api/src/lib.rs` 为空占位；`apps/client/web/app/src/lib/ipc/auth.ts` 手写接口，存在漂移风险。

**推荐增量（v0.1.1 主线）**
1. `contracts_api` 引入：
   - `serde`（derive）
   - `ts-rs = "12.0.1"`
2. 所有跨边界 DTO 在 `contracts_api` 定义并 `#[derive(TS, Serialize, Deserialize)]`。
3. 使用 `#[ts(export)]` + `#[ts(export_to = "...")]` 生成到 `apps/client/web/app/src/lib/bindings/contracts.ts`。
4. Moon 新增 `typegen:contracts` 任务；Web `check`/`lint` 依赖该任务。

**二阶段可选（不是 v0.1.1 必须）**
- 增加 `tauri-specta = 2.0.0-rc.24`，把 command 签名也导出 TS，替代手写 `invoke('xxx')`。

---

### 3) runtime_tauri boundary convergence

**当前现状**
- `packages/adapters/hosts/tauri/src/lib.rs` 为空；`apps/client/native/src-tauri/src/lib.rs` 里有大量 setup/auth/sync/托盘逻辑，host 过重。

**推荐增量（不引入新框架）**
- 不新增依赖，做“职责搬迁”：
  1. `runtime_tauri` 承担：command 注册、事件挂载、状态桥接（AppState facade）
  2. `native/src-tauri/src/lib.rs` 仅保留：builder 组装与生命周期入口
  3. `commands/*` 保持原位置或迁入 `runtime_tauri`，但导出统一注册函数

**为何不新增 DI/actor 框架**
- 当前规模下会增加认知成本且无明确收益；边界问题主要是模块组织，不是运行时能力不足。

---

### 4) moon/just verification workflows

**当前现状**
- 根 `moon.yml` 有 `check/lint/test`，但缺 `typegen`、`fullstack:dev`、统一 `verify` 闭环。
- `Justfile` 通过 `cd` 拼接命令，缺 dotenv 自动加载与跨任务依赖表达。

**推荐增量**
1. moon：新增任务
   - `typegen`（workspace root 执行）
   - `verify`（deps: rust check/lint/test + web check/lint + typegen 一致性检查）
   - `fullstack:dev`（聚合 server + web + tauri）
2. just：
   - 加 `set dotenv-load`
   - recipe 依赖 `verify: typegen ...`
   - 保留已有命令名，减少团队迁移成本

---

### 5) decision ledger docs process

**推荐栈**
- 不加新 docs 引擎；用纯 Markdown + 模板化字段。
- 目录建议：`.planning/decisions/`，命名 `YYYY-MM-DD-<slug>.md`。
- 增加 moon/just 校验任务：
  - 必须字段（Status/Context/Decision/Consequences/Rollout）
  - 每个决策必须回链到 roadmap phase / requirement 编号

**理由**
- 这是流程治理问题，不是渲染问题。先保证“可追踪”和“可验证”，而非引入文档平台复杂度。

## Installation (增量命令)

```bash
# Rust: JWT hardening
cargo add openidconnect@4.0.1 -p native-tauri --registry crates-io

# Rust: contract typegen
cargo add ts-rs@12.0.1 serde@1 --features derive -p contracts_api --registry crates-io

# Optional phase-2 only (command binding generation)
cargo add tauri-specta@2.0.0-rc.24 specta@2.0.0-rc.24 -p native-tauri --registry crates-io
```

## Alternatives Considered

| Category | Recommended | Alternative | Why Not Default (for v0.1.1) |
|----------|-------------|-------------|-------------------------------|
| OIDC/JWT 验签 | `openidconnect 4.0.1` | 仅 `jsonwebtoken` + 手写 JWKS 拉取 | 可行但实现细节多、维护成本高，易遗漏 nonce/issuer/audience 细节。 |
| Rust→TS 类型导出 | `ts-rs 12.0.1` | `tauri-specta` 全量替换 | specta 目前仍 rc，建议二阶段引入，先解决 DTO 漂移主问题。 |
| 文档决策流程 | Markdown ADR + task 校验 | 引入 Backstage/Docusaurus 插件链 | 超出里程碑目标，增加维护面。 |

## What NOT to Add Now

| Avoid Now | Why | Use Instead |
|-----------|-----|-------------|
| 立刻全量切到 `tauri-specta` | rc 版本，且会触发 command 层面较大改造 | v0.1.1 先 `ts-rs` 做 DTO 闭环；specta 作为后续增强。 |
| 引入复杂 DI 容器/actor 框架 | 不能直接解决当前边界漂移，且增加复杂度 | 先做 `runtime_tauri` 模块收敛与注册入口统一。 |
| 新增文档平台依赖（MADR CLI/站点生成器） | 决策沉淀需求是“结构化记录+校验”，不是“发布网站” | Markdown 模板 + moon/just 校验。 |
| 替换 moon 或 just | 当前链路已可用，迁移成本高、收益低 | 在现有工具上补任务图与聚合命令。 |

## Migration / Rollout Order (最小 churn)

1. **先安全（JWT/OIDC 验签）**
   - 改动集中在 `auth.rs`，风险可控、收益最高。
2. **再契约（contracts_api + ts-rs）**
   - 先导出 DTO 类型，替换 TS 手写接口引用。
3. **再工作流（moon/just typegen+verify）**
   - 把第 1/2 步变成自动化门禁，防回退。
4. **再边界收敛（runtime_tauri）**
   - 在行为不变前提下搬迁模块职责，减少入口膨胀。
5. **最后决策账本流程固化**
   - 模板+校验上线，补齐本里程碑所有决策记录。

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| `tauri 2.10.3` | `tauri-plugin-* 2.x` | 与当前仓库一致，保持不变。 |
| `ts-rs 12.0.1` | `serde 1.x` | derive 组合稳定，适合 contracts crate。 |
| `openidconnect 4.0.1` | `reqwest` 客户端 | 支持 discovery + id_token verifier；可与现有 OAuth 流程拼接。 |
| `tauri-specta 2.0.0-rc.24` | `specta 2.0.0-rc.24` | 需版本对齐；建议标记为可选增强。 |

## Sources

- Context7 `/keats/jsonwebtoken` — Validation/claims/JWK 支持（HIGH）
- Context7 `/ramosbugs/openidconnect-rs` — Discovery/JWKS/ID token claims 验证示例（HIGH）
- Context7 `/aleph-alpha/ts-rs` — `#[derive(TS)]`、`#[ts(export)]`、`export_to` 用法（HIGH）
- Context7 `/specta-rs/tauri-specta` — command 收集与 TS 导出方式（MEDIUM，因 rc 状态）
- Context7 `/moonrepo/moon` — task deps/inputs/outputs/runFromWorkspaceRoot（HIGH）
- Context7 `/casey/just` — dotenv-load、recipe deps、shell 设置（HIGH）
- 官方 Tauri v2 文档（Context7 `/websites/v2_tauri_app`）— command 注册/模块化组织（HIGH）
- 仓库现状代码：
  - `apps/client/native/src-tauri/src/commands/auth.rs`
  - `packages/contracts/api/src/lib.rs`
  - `packages/adapters/hosts/tauri/src/lib.rs`
  - `moon.yml` / `Justfile`

---

*Stack research for: milestone v0.1.1 (architecture convergence + production closure)*
*Researched: 2026-04-01*
