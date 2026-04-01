# Pitfalls Research

**Domain:** Brownfield Tauri + SvelteKit + Axum（v0.1.1 安全硬化 / typegen / 边界收敛）  
**Researched:** 2026-04-01  
**Confidence:** HIGH（仓库实证 + Context7 官方文档）

---

## Critical Pitfalls

### Pitfall 1: Tenant 鉴权“看起来可用”但实际上可伪造（`insecure_decode`）

**What goes wrong:**
当前 `servers/api/src/middleware/tenant.rs` 使用 `jsonwebtoken::dangerous::insecure_decode` 提取 `sub`，未验证签名/issuer/audience/exp。攻击者可构造任意 JWT payload，伪造 tenant 身份，导致跨租户数据访问。

**Why it happens:**
历史阶段为了先打通链路（v1）接受了“只 decode 不 verify”。在 brownfield 里，这种“临时方案”最容易被遗忘并进入生产。

**How to avoid:**
1. 将 middleware 改为 `decode + Validation`（验证 `alg/exp/nbf/iss/aud/sub`）。
2. 拒绝与预期算法不匹配的 token（禁止 algorithm confusion）。
3. 把 `tenant_id` 来源切换为“已验签 claims + 后端映射”，不要直接信任 `sub`。
4. 为 token 校验失败写结构化日志（含 request_id，不含敏感 token）。

**Warning signs (detection signal):**
- 代码中出现 `dangerous::insecure_decode`。  
- 401 比例异常低，但跨租户访问告警升高。  
- 伪造 `sub` 的集成测试仍返回 200。

**Phase to address:**
- v0.1.1 Phase A（安全基线硬化）

---

### Pitfall 2: 敏感配置与默认占位值漏入运行时（假安全）

**What goes wrong:**
`apps/client/native/src-tauri/src/commands/config.rs` 和 `commands/auth.rs` 对 `GOOGLE_CLIENT_SECRET`、`GOOGLE_CLIENT_ID` 使用占位默认值（`YOUR_*`）；`servers/api/src/config.rs` 中 `jwt_secret` 也有 dev 默认值。结果是：
- 本地“能跑”但实际未正确加固；
- CI/生产可能带着弱配置上线；
- secret 出现在桌面端配置读取路径中，增加暴露面。

**Why it happens:**
模板项目常用“兜底默认值”降低上手门槛，但里程碑升级到“生产闭环”后，默认值策略需要切换为 fail-fast。

**How to avoid:**
1. 区分 dev/prod：生产环境遇到占位值或空值直接启动失败。  
2. 不在客户端配置命令中返回 client secret（前端只需 client_id 与 API endpoint）。  
3. 将 refresh_token 等高敏感数据迁移到 Stronghold（插件生态建议）而非普通 store。  
4. 加入 `verify` 任务：扫描 `YOUR_`、`dev-secret-change-in-production`、空 token。

**Warning signs (detection signal):**
- 日志出现 `client_id_len=0` 或 `api_url` 为空。  
- 仓库 grep 出现 `YOUR_GOOGLE_CLIENT_SECRET`、`dev-secret-change-in-production`。  
- 前端可通过 `get_config` 拿到 secret。

**Phase to address:**
- v0.1.1 Phase A（安全基线硬化）

---

### Pitfall 3: 本地 `.env` 与绝对路径依赖导致跨环境回归

**What goes wrong:**
`apps/client/native/src-tauri/src/lib.rs` 在 setup 中写死项目绝对路径 `/Users/.../tauri-sveltekit-axum-moon-template` 并尝试 `set_current_dir` + `dotenv_override`。这会在 CI、其他开发机、打包后安装环境直接失效。

**Why it happens:**
brownfield 常有“为本机调试临时加的路径”，后续未收敛到 runtime/path abstraction。

**How to avoid:**
1. 删除硬编码路径，统一用 Tauri path API（app data/config dir）。  
2. 开发态可选读取 `.env`，发布态必须通过平台配置注入。  
3. 在 `runtime_tauri` 里集中配置加载策略，native host 仅负责启动与依赖注入。

**Warning signs (detection signal):**
- 代码中出现绝对路径（`/Users/`、`C:\\`）。  
- 打包后首次启动日志出现 `failed to load .env` 后功能异常。  
- 不同机器需要“手工改路径”才能运行。

**Phase to address:**
- v0.1.1 Phase A（路径可移植性） + Phase C（runtime 边界收敛）

---

### Pitfall 4: typegen 管线“半自动”导致 Rust/TS 契约漂移

**What goes wrong:**
`packages/contracts/api` 当前仍是 placeholder（仅注释），如果直接在 TS 与 Rust 各自手写 DTO，短期可用，后续出现字段名/可空性/枚举值漂移，引发运行时隐性 bug（尤其是 IPC 与 HTTP 双边界）。

**Why it happens:**
brownfield 中最常见的是“先手写，后补 typegen”，但后补时常缺少单一真实源（single source of truth）。

**How to avoid:**
1. 规定 `contracts_api` 为唯一契约源；Rust/TS 均由其生成。  
2. moon 增加显式 `typegen` 任务并作为 `check-all/test-all` 前置依赖。  
3. CI 增加“生成后无 diff”守门（防止开发者忘记提交生成产物）。  
4. 对破坏性变更采用版本化策略（至少在 changelog/decision log 标记）。

**Warning signs (detection signal):**
- `contracts_api/src/lib.rs` 仍为空或仅注释。  
- PR 里修改了 Rust DTO 但无 TS 类型更新。  
- E2E 报错集中在序列化/反序列化边界。

**Phase to address:**
- v0.1.1 Phase B（契约与 typegen 闭环）

---

### Pitfall 5: runtime 边界收敛失败，native host 继续膨胀

**What goes wrong:**
`apps/client/native/src-tauri/src/lib.rs` 当前承载：DB 初始化、迁移、sync 启动、refresh timer、panic hook、tray 菜单、window close 行为等多类职责。若 v0.1.1 继续在此叠加逻辑，会导致：
- 回归面扩大（任一改动影响启动路径）；
- 可测性差（难做细粒度单测）；
- runtime_tauri 形同虚设。

**Why it happens:**
brownfield 默认“就近改动”偏向在入口文件堆逻辑，短期快，长期不可维护。

**How to avoid:**
1. 明确边界：`lib.rs` 仅编排；业务初始化移入 `runtime_tauri`。  
2. 拆分 startup pipeline（config/auth/sync/ui lifecycle）并可独立测试。  
3. 给 `runtime_tauri` 增加最小 API 面，禁止反向依赖 host 细节。

**Warning signs (detection signal):**
- `lib.rs` 持续增长且新增 feature 直接改入口。  
- 新逻辑无法在不启动 Tauri 的情况下测试。  
- 代码评审频繁出现“顺手在 setup 加一下”。

**Phase to address:**
- v0.1.1 Phase C（运行时边界收敛）

---

### Pitfall 6: Moon/Just 工作流补全不彻底，文档与执行漂移

**What goes wrong:**
路标要求 `fullstack:dev / typegen / verify` 统一入口，但当前仓库 moon 任务以 cargo check/test 为主，尚未形成“单命令可复现全链路”。结果是：
- 新成员按文档无法一键跑通；
- 本地通过与 CI 通过标准不一致；
- 规划文档状态与真实任务图不一致（planning drift）。

**Why it happens:**
里程碑是后补式治理，若只补代码不补任务与文档，漂移会在下一轮迭代放大。

**How to avoid:**
1. 建立统一入口：`fullstack:dev`（web+api+native 编排）、`typegen`、`verify`。  
2. `verify` 必须覆盖：格式、lint、check、test、typegen-diff、secret/default-value scan。  
3. 将 `.planning/ROADMAP.md` 的 phase success criteria 映射到可执行任务（任务名即验收项）。  
4. 每次 phase 结束同步更新 `STATE.md` 与任务矩阵。

**Warning signs (detection signal):**
- CI 能过但本地无等价命令；或反之。  
- ROADMAP 标记完成，但对应 task 不存在。  
- 新人 README 走不通，需要口头补步骤。

**Phase to address:**
- v0.1.1 Phase D（任务体系补全） + Phase E（决策账本）

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| `insecure_decode` 先上线 | 快速打通租户链路 | 高危越权风险 | 仅限本地 PoC，禁止进入生产分支 |
| 客户端配置返回 secret | 调试方便 | 凭据暴露面扩大 | Never |
| 入口 `lib.rs` 集中堆功能 | 改起来快 | 回归半径持续扩大 | 仅限临时 hotfix，需后续拆分 ticket |
| 手写双端类型 | 无需搭 typegen | 长期契约漂移 | 一次性 spike 可接受，合入主干前必须回归 typegen |
| 文档先不更新 | 节省当下时间 | 团队执行分叉 | Never（本里程碑目标即“决策沉淀”） |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Axum tenant middleware × OAuth tokens | 只读 `sub` 不验签 | 使用 `decode + Validation` 并校验 iss/aud/exp |
| Tauri plugin-store × auth session | 把 refresh token 明文长期存储 | token 分级存储；高敏感迁移 Stronghold |
| Tauri setup × 环境配置 | 依赖项目根 `.env` + 绝对路径 | 发布态走标准配置注入 + path API |
| contracts_api × 前端类型 | contract crate 仍 placeholder | 先固化 contract schema，再生成 TS/Rust 类型 |
| moon tasks × roadmap criteria | 只有 cargo 子集任务 | 增加 fullstack/typegen/verify 与文档双向绑定 |

---

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| 启动阶段串行做太多初始化 | 冷启动慢、偶发超时 | 分层初始化（关键路径最小化，后台异步延迟加载） | 功能增加后立即恶化 |
| typegen 未增量化 | CI 时间显著上升 | 基于输入变更触发生成，缓存产物 | 合同规模扩大后明显 |
| 单入口文件高耦合 | 小改动引发大范围重编译/回归 | runtime 边界拆分 | 中期持续恶化 |

---

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| `dangerous::insecure_decode` 用于权限决策 | 伪造 token 越权 | 强制验签与 claim 验证 |
| 占位默认 secret 可运行 | 生产弱密钥/错误配置上线 | prod fail-fast + verify 扫描 |
| 客户端暴露 `google_client_secret` | 凭据泄露 | 客户端仅保留 public config |
| 普通 store 长期存放高敏 token | 本地泄露风险 | Stronghold/OS 安全存储 + 轮换策略 |
| permissive CORS 长期保留 | 跨域攻击面扩大 | dev/prod 分离配置，生产最小化策略 |

---

## Workflow / Documentation Drift Pitfalls

| Pitfall | Impact | Guardrail |
|---------|--------|-----------|
| ROADMAP 状态与真实代码不一致 | 决策失真，后续 phase 误判 | 每次 phase 完成后同步 STATE/ROADMAP/任务清单 |
| 任务名不稳定（同义多命令） | 团队执行习惯分裂 | 只保留官方入口：fullstack:dev/typegen/verify |
| 决策仅在聊天记录，不落盘 | 关键约束丢失 | 维护决策账本（实现/延期/放弃+原因） |

---

## "Looks Done But Isn't" Checklist

- [ ] **Tenant middleware:** 已能解析 token，但未验证签名/aud/iss/exp。  
- [ ] **Security hardening:** 已有 env 读取，但仍允许占位默认值进入运行。  
- [ ] **Typegen:** 有 contracts crate，但未接入 CI diff 守门。  
- [ ] **Boundary refactor:** runtime_tauri crate 存在，但 host 入口职责未减。  
- [ ] **Workflow:** moon 有 check/test，但无 fullstack:dev/typegen/verify 统一闭环。  
- [ ] **Docs:** phase 标记完成，但无可执行命令映射到 success criteria。

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| 伪造 token 越权已上线 | HIGH | 立即切验签；轮换密钥；审计访问日志；补跨租户回归测试 |
| secret 默认值进入生产 | HIGH | 立即失效旧凭据；添加启动阻断；补 CI secret-scan |
| type 漂移导致线上序列化错误 | MEDIUM | 回滚到最近一致版本；补 typegen 单源；加 contract compatibility test |
| 边界重构引发启动回归 | MEDIUM | 分阶段拆分 + feature flag；保留旧路径回退窗口 |
| 文档/任务漂移 | LOW-MEDIUM | 统一任务入口；将验收项改为“命令可验证”并补自动检查 |

---

## Pitfall-to-Phase Mapping (v0.1.1)

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| JWT 未验签导致租户伪造 | Phase A 安全基线 | 伪造 token E2E 必须 401；合法 token 200 |
| 默认占位 secret / 客户端 secret 暴露 | Phase A 安全基线 | 生产配置缺失时启动失败；`get_config` 不含 secret |
| 绝对路径/.env 依赖 | Phase A + C | 跨机器/CI/打包安装场景一致通过 |
| contracts_api 空壳、双端类型漂移 | Phase B typegen | typegen 后 git diff 必须为空 |
| host 入口职责膨胀 | Phase C 边界收敛 | `lib.rs` 仅编排；新增 feature 不再直接改入口 |
| moon/文档执行漂移 | Phase D + E | `verify` 一条命令覆盖里程碑验收项；STATE/ROADMAP 同步 |

---

## Sources

### HIGH confidence
- Repository evidence (current milestone code):  
  - `servers/api/src/middleware/tenant.rs`  
  - `apps/client/native/src-tauri/src/commands/config.rs`  
  - `apps/client/native/src-tauri/src/lib.rs`  
  - `apps/client/native/src-tauri/src/commands/auth.rs`  
  - `packages/contracts/api/src/lib.rs`  
  - `moon.yml` + module-level `moon.yml`
- Context7 / Official docs:  
  - jsonwebtoken (`dangerous::insecure_decode` does not verify signature/claims)  
  - Tauri v2 permissions/capabilities model（deny-by-default + capability attachment）  
  - Tauri plugins workspace（Stronghold for secure secret storage）

### MEDIUM confidence
- 具体“最佳实践落地方式”（如 token 分级策略、任务命名细节）属于工程策略层，需要在本里程碑 design review 中最终定稿。

---

*Pitfalls research for: brownfield Tauri+SvelteKit+Axum v0.1.1*  
*Researched: 2026-04-01*
