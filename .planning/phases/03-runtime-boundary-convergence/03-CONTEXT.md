# Phase 3: Runtime 边界收敛 - Context

**Gathered:** 2026-04-02
**Status:** Ready for planning

<domain>
## Phase Boundary

核心业务规则不依赖任何 host/protocol/chain crate。adapters 只做外部世界翻译，不承载业务策略。新增 capability 通过 feature 模块组合 core + contracts + adapters 实现，不绕过边界。

本阶段只做架构收敛（迁移、解耦、建立 crate 边界），不实现业务功能（Phase 4）。

</domain>

<decisions>
## Implementation Decisions

### Port 实现归属
- **D-01:** Port 实现（`TenantAwareSurrealDb`, `TursoDb`）从 `servers/api/src/ports/` 迁移到 `packages/adapters/storage/`
- **D-02:** 每个实现一个独立 crate：`packages/adapters/storage/surrealdb` 和 `packages/adapters/storage/libsql`
- **D-03:** `servers/api` 通过 Cargo path dependency 引用 adapter crates
- **D-04:** 迁移后 `servers/api/src/ports/` 删除或清空，server 只做 HTTP runtime

### usecases 解耦
- **D-05:** `usecases` crate 不再依赖 `contracts_api`
- **D-06:** `usecases` 定义自己的内部输入/输出类型（如 `InitTenantInput` / `InitTenantOutput`）
- **D-07:** Route handler（`servers/api/src/routes/`）负责 `contracts_api` DTO ↔ `usecases` 内部类型的映射
- **D-08:** `usecases` 只依赖 `domain`（Port traits）

### runtime_tauri 职责
- **D-09:** `packages/adapters/hosts/tauri`（runtime_tauri）承载 Tauri command handler 桥接
- **D-10:** `runtime_tauri` 依赖 `domain` + `usecases`，定义 `#[tauri::command]` 函数调用 usecases
- **D-11:** `apps/client/native/src-tauri`（native-tauri）只做 builder/bootstrap + plugin 注册，不直接包含业务 command handler
- **D-12:** native-tauri 的 `tauri::Builder` 通过 `.invoke_handler(tauri::generate_handler![...])` 注册 runtime_tauri 导出的 commands

### 边界强制机制
- **D-13:** CI 硬性检查：repo:verify 增加边界检查 task，验证依赖方向（core 不能依赖 adapters/hosts，contracts 不能依赖 core/adapters）
- **D-14:** `cargo-deny` 配置 `[dependencies]` 规则，限制非法 crate 依赖方向
- **D-15:** `.agents/rubrics/boundary-compliance.md` 编写语义边界规范（adapter 不含业务规则，usecases 不依赖宿主）
- **D-16:** Agent code review 时检查 rubric，CI 做硬性门禁

### features/ 模块定位
- **D-17:** 每个 feature（auth, admin, counter, agent 等）是一个独立 Cargo crate
- **D-18:** Feature crate 组合 core（domain + usecases）+ contracts + adapters，不直接依赖宿主 runtime
- **D-19:** Feature crate 定义该 feature 的完整能力边界：use case 入口 + 所需 adapter 绑定
- **D-20:** Phase 3 创建 features 骨架（Cargo.toml + src/lib.rs），实际实现在 Phase 4

### Cargo workspace 更新
- **D-21:** 新增 workspace members：`packages/adapters/storage/surrealdb`, `packages/adapters/storage/libsql`, `packages/adapters/hosts/tauri`（已有但需充实）, 各 features crates
- **D-22:** Root `Cargo.toml` workspace dependencies 统一管理 adapter 和 feature 的公共依赖版本

### the agent's Discretion
- `packages/adapters/storage/surrealdb` 和 `libsql` crate 的内部文件组织
- `cargo-deny` 的具体规则配置细节
- `runtime_tauri` 中 command handler 的命名和分组方式
- Feature crates 的具体命名（`feature-auth`, `feature-admin` 还是 `auth-feature`, `admin-feature`）
- 依赖方向检查的具体实现（自定义脚本 vs cargo-deny vs deny.toml）

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase & Requirements
- `.planning/ROADMAP.md` §Phase 3 — Phase goal, success criteria, dependencies
- `.planning/REQUIREMENTS.md` §RUNTIME-01, §RUNTIME-02, §RUNTIME-03 — Acceptance criteria

### Prior Phase Decisions
- `.planning/phases/01-repo-structure-toolchain/01-CONTEXT.md` — 目录结构、moon tasks、Cargo workspace 格式
- `.planning/phases/02-contracts-typegen/02-CONTEXT.md` — contracts crate 结构、typegen 管线、DTO 命名规范

### Architecture Reference
- `docs/ARCHITECTURE.md` — Layer boundaries、hexagonal 原则、growth path

### Existing Code (migration targets)
- `servers/api/src/ports/` — `TenantAwareSurrealDb`, `TursoDb` 实现待迁移
- `servers/api/src/routes/tenant.rs` — `InitTenantRequest`/`Response` 映射点
- `servers/api/src/routes/health.rs` — `HealthResponse` 映射点
- `packages/adapters/hosts/tauri/src/lib.rs` — 当前空壳，待充实
- `apps/client/native/src-tauri/src/lib.rs` — native-tauri entry point，需重构 command 注册

### Cargo Configuration
- `Cargo.toml` — workspace members 和 workspace.dependencies
- 各 crate 的 `Cargo.toml` — 现有依赖关系

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `packages/core/domain/src/ports/` — Port traits（`LibSqlPort`, `SurrealDbPort`）已定义，adapter 实现需符合这些 trait
- `packages/contracts/api/` — contracts_api crate 已就位，route handler 将从这里导入 DTO
- `moon.yml` — repo:verify task 可扩展加入边界检查

### Established Patterns
- Cargo workspace 使用 path dependencies 和 workspace.dependencies 统一版本
- moon tasks 使用 `command` + `inputs` + `deps`
- Serde + utoipa + ts-rs derive 宏共存于同一 struct

### Integration Points
- `Cargo.toml` root — 添加新 workspace members
- `moon.yml` — repo:verify 增加边界检查子任务
- `servers/api/src/lib.rs` — server 路由注册，迁移后引用新 adapter crates
- `apps/client/native/src-tauri/src/lib.rs` — native-tauri builder 注册 runtime_tauri commands

### Dependency Graph (Current → Target)

**Current:**
```
domain ← usecases ← servers/api
domain ← usecases ← native-tauri
contracts_api ← usecases  (不合规)
contracts_api ← servers/api
port impl 在 servers/api  (不合规)
```

**Target:**
```
domain ← usecases (usecases 不再依赖 contracts_api)
domain ← adapters/storage/surrealdb
domain ← adapters/storage/libsql
domain ← runtime_tauri ← native-tauri
usecases ← adapters/storage/*
usecases ← runtime_tauri
usecases ← servers/api
contracts_api ← servers/api (route handler 做映射)
contracts_api ← features/*
core + adapters + contracts ← features/*
```

</code_context>

<specifics>
## Specific Ideas

- 迁移 port 实现时保持 trait 实现签名不变，只改 crate 归属和 mod 路径
- usecases 内部类型命名与 contracts_api 类型平行但区分（如 `InitTenantInput` vs `InitTenantRequest`）
- runtime_tauri 的 command handler 按 feature 分 module（auth commands, counter commands, admin commands）
- features crates 的命名规范：`feature-{name}`（如 `feature-auth`, `feature-admin`）
- 边界检查可以利用 `cargo tree` 输出验证依赖方向

</specifics>

<deferred>
## Deferred Ideas

- Phase 4 才做 features 的实际业务实现
- adapters/storage 的具体 SurrealDB/LibSQL 配置和连接池管理 — Phase 4 随功能实现
- adapters/auth/（OAuth, passkey）的实际实现 — Phase 4
- telemetry adapters（otel, tracing）— 按需
- 完整的 cargo-deny 配置（license, advisory 等）— 后续迭代

</deferred>

---

*Phase: 03-runtime-boundary-convergence*
*Context gathered: 2026-04-02*
