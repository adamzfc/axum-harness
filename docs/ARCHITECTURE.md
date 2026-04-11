```markdown
# 📁 项目目录架构：职责边界 · 依赖方向 · 注释规范

> **设计原则**：架构即约束，注释即契约，决策即文档。
> 本文档仅保留**目录结构设计、职责定义、依赖规则**；架构哲学、工具链决策、存储策略等详见 [spec.md](./spec.md)。

---

## 📦 目录架构总览
```

tauri-sveltekit-axum-turso-boilerplate/
│
├── 📦 根配置层 · [必须] 工具链钉死 + 任务编排 + 工作区定义
│ │ # 职责: 开发环境配置集中管理
│ │ # 禁止: 散落业务逻辑或环境配置
│ │
│ ├── .mise.toml # [mise] 工具版本管理 [必须]
│ │ # 职责: 钉死 Rust/Node/bun/moon/just/biome 等工具版本
│ │ # 验证: `mise doctor` 必须通过
│ │
│ ├── justfile # [just] 统一命令入口 [必须]
│ │ # 职责: 人类+Agent 的唯一命令入口，薄层转发到 moon
│ │ # 验证: `just --list` 必须列出所有可用命令
│ │
│ ├── moon.yml # [moon] monorepo 任务编排 [必须]
│ │ # 职责: 定义项目图、任务依赖、affected 计算规则
│ │ # 验证: `moon ci:affected --base=main` 必须正确计算变更影响
│ │
│ ├── Cargo.toml # [cargo] Rust workspace 根 [必须]
│ │ # 职责: 定义 workspace members, 统一依赖版本 (通过 cargo-hakari)
│ │ # 验证: `cargo hack check --workspace` 必须通过
│ │
│ ├── package.json # [bun] JS workspace 根 [必须]
│ │ # 职责: 定义前端工作区、共享 devDependencies
│ │ # 验证: `bun --filter '!./apps/*' run build` 必须通过
│ │
│ ├── bun-workspace.yaml # [bun] 工作区定义 [必须]
│ │ # 职责: 声明哪些目录属于 bun workspace
│ │ # 验证: `bun --filter '*' echo $PWD` 必须只列出前端目录
│ │
│ ├── biome.json # [biome] 前端 lint/format [必须]
│ │ # 职责: 统一 Svelte/TS/JS 的代码风格规则
│ │ # 验证: `bun exec biome check .` 必须零警告
│ │
│ ├── rust-toolchain.toml # [rustup] 工具链版本 [必须]
│ │ # 职责: 钉死 Rust channel + components
│ │ # 验证: `rustc --version` 必须匹配
│ │
│ ├── .cargo/config.toml # [cargo] 全局配置 [必须]
│ │ # 职责: 定义 registry mirror, alias, target-dir, linker
│ │ # 验证: `cargo build --workspace` 必须使用配置的输出目录
│ │
│ ├── .gitattributes # [git] 文件属性 [必须]
│ │ # 职责: 定义行尾符、二进制文件处理、diff 驱动
│ │ # 验证: `git check-attr -a <file>` 必须返回预期属性
│ │
│ └── .gitignore # [git] 全局忽略 [必须]
│ # 职责: 忽略构建产物、环境文件、本地配置
│ # 验证: `git status --porcelain` 必须干净
│
├── 🤖 agent/ · [必须] Agent Harness 约束定义
│ │ # 职责: 定义 Agent 生成代码的约束边界
│ │ # 禁止: 业务逻辑代码出现在此目录
│ │
│ ├── README.md # [必须] Agent 首要阅读入口
│ ├── codemap.yml # [必须] 模块边界 + 依赖约束 [核心]
│ ├── boundaries.md # [必须] 目录修改边界定义
│ ├── constraints/ # [必须] 约束规则集合
│ │ ├── dependencies.yaml # [必须] 依赖白/黑名单
│ │ ├── patterns.yaml # [必须] 禁止代码模式
│ │ ├── contracts.yaml # [必须] 契约变更流程
│ │ └── storage-policy.yaml # [必须] 存储策略约束 [关键]
│ ├── prompts/ # [必须] Agent 生成操作模板
│ │ ├── add-module.md / add-endpoint.md / add-sync-strategy.md / split-service.md
│ ├── checklists/ # [必须] 关键操作检查清单
│ │ ├── schema-change.md / migration.md / sync-conflict.md / release.md
│ └── templates/ # [必须] 代码生成模板
│ ├── module/ # [必须] 新模块脚手架 (domain/application/ports/contracts/sync)
│ └── bff-endpoint/ # [必须] BFF 端点模板
│
├── 🌐 apps/ · [必须] 多端应用入口 (纯展示层 · 零业务逻辑)
│ │ # 依赖方向: apps/_ → packages/contracts/sdk + packages/ui
│ │ # 禁止: 直接调用 services/_ 或 packages/adapters/_
│ │
│ ├── web/ # [必须] SvelteKit 5 Web 应用
│ │ ├── src/
│ │ │ ├── routes/ # [必须] 页面路由 (public/auth/tenant/platform/api)
│ │ │ ├── lib/
│ │ │ │ ├── api/ # [必须] 仅放 generated SDK
│ │ │ │ ├── auth/ # [必须] 前端鉴权守卫
│ │ │ │ ├── tenancy/ # [必须] 租户上下文管理
│ │ │ │ ├── stores/ # [必须] 全局状态 (Svelte 5 runes)
│ │ │ │ ├── sync/ # [必须] 前端同步协调器
│ │ │ │ └── components/ # [可选] 应用级组件
│ │ │ ├── hooks.server.ts # [必须] SSR 鉴权 + 租户注入
│ │ │ └── app.html # [必须] HTML 模板
│ │ ├── tests/ # [必须] e2e + unit 测试
│ │ ├── package.json / svelte.config.js / vite.config.ts
│ │ └── .gitkeep
│ │
│ ├── mobile/ # [预留] 移动端 (Tauri2 Mobile)
│ │ ├── src/lib/api/ # 仅用 generated SDK
│ │ ├── src/lib/sync/ # 移动端同步策略 (OfflineFirst)
│ │ └── package.json / .gitkeep
│ │
│ └── desktop/ # [必须] Tauri v2 桌面应用
│ ├── src/ # 前端部分 (SvelteKit, 与 web 共享逻辑)
│ │ ├── lib/
│ │ │ ├── tauri/ # [必须] Tauri 特有能力封装
│ │ │ ├── sync/ # 桌面端同步协调 (调用 Rust commands)
│ │ │ └── .gitkeep
│ │ └── .gitkeep
│ │
│ ├── src-tauri/ # [必须] Tauri Rust 后端
│ │ ├── capabilities/ # [必须] 细粒度权限 (Tauri v2)
│ │ ├── src/
│ │ │ ├── commands/ # [必须] 桌面壳命令 (store/sync)
│ │ │ ├── deep_link/ # [必须] OIDC / magic-link / 自定义协议
│ │ │ ├── system_tray/ # [可选] 系统托盘集成
│ │ │ └── main.rs # [必须] Tauri 入口
│ │ ├── tauri.conf.jsonc / Cargo.toml
│ │ └── .gitkeep
│ │
│ └── package.json
│
├── ⚙️ servers/ · [必须] 服务端组合层 (聚合 · 协议转换 · 视图组装)
│ │ # 依赖方向: servers/_ → services/_ (via trait) + packages/core + packages/contracts
│ │ # 禁止: 包含领域业务逻辑，仅做聚合/转换/路由
│ │ # 验证: 每个 server 必须可独立 `cargo build -p <server-name>`
│ │
│ ├── bff/ # [必须] Backend For Frontend 层
│ │ │ # 职责: 按端聚合视图模型，处理端特定转换 (不写业务规则)
│ │ │ # 依赖: services/_ (trait only) + packages/contracts
│ │ │
│ │ ├── web-bff/ # [必须] Web 端聚合 (Axum)
│ │ │ ├── src/
│ │ │ │ ├── handlers/ # [必须] 端特定视图模型组装
│ │ │ │ ├── adapters/ # [必须] 调用 services 的 trait 实现
│ │ │ │ ├── middleware/ # [必须] BFF 层中间件
│ │ │ │ ├── config.rs / main.rs # [必须] 配置 + 入口
│ │ │ │ └── .gitkeep
│ │ │ ├── Cargo.toml # [必须] BFF 依赖声明
│ │ │ ├── openapi.yaml # [必须] BFF 的 OpenAPI 契约
│ │ │ └── .gitkeep
│ │ │
│ │ ├── mobile-bff/ # [预留] 移动端聚合
│ │ │ ├── src/
│ │ │ │ ├── handlers/ / adapters/ / middleware/
│ │ │ │ └── middleware/offline*cache.rs.gitkeep
│ │ │ ├── Cargo.toml / openapi.yaml / .gitkeep
│ │ │
│ │ └── admin-bff/ # [必须] 管理端聚合
│ │ ├── src/handlers/
│ │ │ ├── platform_admin/ # 跨租户操作 (需 PlatformAdmin 角色)
│ │ │ └── tenant_admin/ # 租户内管理 (需 Admin 角色)
│ │ ├── Cargo.toml / openapi.yaml / .gitkeep
│ │
│ ├── composition/ # [可选] 共享服务组合逻辑 (跨 BFF 复用)
│ │ │ # 职责: 定义通用组合 trait + 实现，供多个 BFF 复用
│ │ │ # 禁止: 包含业务规则，仅做数据聚合/转换
│ │ │
│ │ ├── src/
│ │ │ ├── traits/ # 组合 trait 定义
│ │ │ ├── composers/ # 具体组合实现
│ │ │ └── lib.rs
│ │ ├── Cargo.toml # 依赖: packages/core + packages/contracts
│ │ └── .gitkeep
│ │
│ ├── indexer/ # [必须] 协议事件索引服务 (原 services/indexer 合并)
│ │ │ # 职责: 按协议拆分拉取源 → 标准化为业务 DTO → 写入 Turso
│ │ │ # 依赖: packages/web3 + packages/core + packages/adapters/turso
│ │ │ # 禁止: 包含业务逻辑，仅做事件拉取/转换/写入
│ │ │
│ │ ├── src/
│ │ │ ├── sources/ # 按协议拆分拉取源 (nostr/farcaster/evm/ton)
│ │ │ ├── transformers/ # 原始事件 → 业务 DTO
│ │ │ ├── sinks/ # 写入 Turso / 缓存 / 触发领域事件
│ │ │ └── lib.rs
│ │ ├── Cargo.toml
│ │ └── .gitkeep
│ │
│ └── api-gateway/ # [Phase 2+] 统一网关 (可选)
│ │ # 职责: 鉴权/限流/路由分发到各 BFF，不处理业务
│ │
│ ├── src/
│ │ ├── middleware/ # 鉴权/限流/日志/链路追踪
│ │ ├── router/ # 路由分发 (path → bff)
│ │ └── main.rs
│ ├── Cargo.toml
│ └── .gitkeep
│
├── ⚙️ services/ · [必须] 领域服务层 (模块化核心 · 独立业务边界)
│ │ # 依赖方向: services/* → packages/core + packages/contracts
│ │ # 禁止: services 之间直接依赖，必须通过 contracts/events 通信
│ │ # 验证: 每个服务必须可独立 `cargo build -p <service>` + `cargo test -p <service>`
│ │
│ ├── user/ # [必须] 用户域: 认证/资料/权限/会话
│ │ ├── src/
│ │ │ ├── domain/ # [必须] 纯领域逻辑
│ │ │ ├── application/ # [必须] 用例逻辑 (纯函数)
│ │ │ ├── ports/ # [必须] 外部依赖抽象
│ │ │ ├── contracts/ # [必须] 稳定契约定义
│ │ │ └── sync/ # [必须] 同步策略定义
│ │ ├── tests/ / migrations/ / Cargo.toml / README.md
│ │ └── .gitkeep
│ │
│ ├── tenant/ # [必须] 多租户域: 隔离策略 + 成员管理
│ │ ├── src/domain/ / application/ / ports/ / contracts/ / sync/
│ │ ├── tests/ / migrations/ / Cargo.toml / .gitkeep
│ │
│ ├── counter/ # [必须] 【黄金示例】最小完整模块
│ │ │ # 用途: Agent 复制此模块创建新业务模块的模板
│ │ │ # 验证: `cargo test -p counter` 必须 100% 通过
│ │ │
│ │ ├── src/domain/ / application/ / ports/ / contracts/ / sync/ / lib.rs
│ │ ├── tests/unit/ / integration/ / contract/ / sync/
│ │ ├── migrations/ / Cargo.toml / README.md / .gitkeep
│ │
│ ├── admin/ # [必须] 平台管理域: 跨租户操作 + 审计
│ │ ├── src/ / tests/ / migrations/ / Cargo.toml / .gitkeep
│ │
│ └── event-bus/ # [必须] 事件总线抽象 + Outbox 实现
│ ├── src/
│ │ ├── ports/ # [必须] EventBus trait
│ │ ├── adapters/ # [必须] 具体实现 (feature 切换)
│ │ │ ├── memory/ # Phase 1: 开发/测试用
│ │ │ └── nats/ # Phase 2+: NATS JetStream
│ │ ├── outbox/ # [必须] Outbox Pattern 实现
│ │ └── config.rs # [必须] 事件总线配置
│ ├── Cargo.toml # [必须] features = ["memory", "nats"]
│ └── .gitkeep
│
├── 🌐 packages/ · [必须] 共享层 (跨服务复用 · 无业务逻辑)
│ │ # 依赖方向: packages/_ → 无 (最底层) 或 packages/core
│ │ # 禁止: 包含业务逻辑或具体基础设施实现
│ │ # 验证: 所有 packages/_ 必须可独立 `cargo build -p <package>`
│ │
│ ├── contracts/ # [必须] 【唯一真理源】契约定义 + 代码生成
│ │ │ # 职责: HTTP/Event/DTO 契约的唯一来源, CI 自动生成多语言 SDK
│ │ │ # 验证: `just gen-openapi` + `just gen-frontend-sdk` 必须零 diff
│ │ │
│ │ ├── http/ # [必须] HTTP API 契约
│ │ ├── events/ # [必须] 事件契约 (AsyncAPI 3.1)
│ │ ├── jsonschema/ # [必须] Payload Schema
│ │ └── sdk-gen/ # [必须] 代码生成配置
│ │
│ ├── core/ # [必须] 【核心抽象】纯 Rust trait + 基础类型
│ │ │ # 职责: 定义系统级抽象 (TenantId/Error/Config/Telemetry)
│ │ │ # 依赖: 无 (最底层)
│ │ │
│ │ ├── kernel/ # [必须] 基础类型 (TenantId/UserId/Error/Cursor)
│ │ ├── platform/ # [必须] 平台能力抽象 (config/telemetry/clock)
│ │ └── .gitkeep
│ │
│ ├── adapters/ # [必须] 【外部依赖适配】基础设施实现
│ │ │ # 设计: 只实现 packages/core 定义的 trait, 不写业务逻辑
│ │ │ # 依赖方向: adapters/\_ → packages/core + external crates
│ │ │ # 禁止: adapters 被业务模块直接依赖 (必须通过 ports/ 抽象)
│ │ │
│ │ ├── store/ # [必须] 前端本地配置存储 (Tauri Store)
│ │ ├── turso/ # [必须] Turso 核心适配器 (跳过 libSQL)
│ │ ├── surreal/ # [可选] SurrealDB 适配器 (实验/复杂模型)
│ │ ├── cache/ # [必须] 缓存抽象 (Moka + Redis 可选)
│ │ ├── auth/ # [必须] 认证适配器
│ │ └── storage/ # [必须] 文件存储适配器
│ │
│ ├── 🌐 web3/ # [必须] 去中心化协议适配层 (独立 workspace)
│ │ │ # 职责: 协议 SDK 封装、链上交互 trait、中继客户端、签名验证
│ │ │ # 依赖: packages/core + protocol-specific crates
│ │ │ # 禁止: 包含业务逻辑或中心化 DB 调用
│ │ │ # 验证: `cargo build -p web3 --features nostr` 必须成功
│ │ │
│ │ ├── README.md # 协议集成原则 + 信任边界说明
│ │ ├── traits/ # [必须] 统一抽象 (IdentityProvider/StatePublisher)
│ │ ├── at-protocol/ # AT Protocol 封装
│ │ ├── farcaster/ # Farcaster Hub/Signer/Frame 封装
│ │ ├── nostr/ # Nostr 中继客户端 + 签名验证
│ │ ├── evm/ # EVM 基础 (Base/其他 L2 共享)
│ │ │ ├── base/ # Base L2 特定配置
│ │ │ └── contracts/ # 生成的 ABI + Rust bindings (ethers/alloy)
│ │ ├── ton/ # TON SDK 封装
│ │ ├── solana/ # Solana 程序交互
│ │ ├── indexer/ # [必须] 链下索引器 (解析事件 → 写入 Turso)
│ │ └── Cargo.toml # 各协议默认 feature = "stub", 按需启用
│ │
│ ├── sdk/ # [必须] 【多语言 SDK】自动生成输出 (git-ignored)
│ │ │ # 职责: 存储由 contracts/ 自动生成的 SDK
│ │ │ # 禁止: 手动修改此目录内容 (必须由 CI 生成)
│ │ │
│ │ ├── typescript/ # 前端用 (由 ts-rs + openapi-gen 生成)
│ │ ├── rust/ # 其他 Rust 服务用 (由 prost/utoipa 生成)
│ │ └── .gitkeep
│ │
│ └── ui/ # [必须] 【共享 UI】Svelte 组件库 + design tokens
│ │ # 职责: 跨端复用的基础组件 + 主题配置
│ │ # 禁止: 包含业务逻辑或端特定代码
│ │
│ ├── src/
│ │ ├── components/ # 基础组件 (Button/Input/Modal)
│ │ ├── layouts/ # 布局组件 (TenantLayout/AdminLayout)
│ │ └── themes/ # 主题配置 (light/dark/tenant-branded)
│ ├── package.json / .gitkeep
│
🌐 infra/ · [必须] 基础设施声明层 (云原生就绪 · 轻量起步)
│ # 原则: 声明式配置 + 阶段化部署, 业务代码零感知
│ # 验证: `just deploy dev` 启动 compose | `just deploy prod` 部署 k3s
│
├── docker/ # [必须] 本地开发与 CI 缓存 (Podman 替代docker)
│ │ # 职责: 仅用于开发环境与 CI 镜像构建
│ │ # 禁止: 用于生产环境部署
│ │
│ ├── compose/
│ │ ├── app.yaml # 主应用 + 依赖 (turso/nats-rs/otel 开发镜像)
│ │ └── observability.yaml # 轻量可观测 (OpenObserve + Vector)
│ ├── Dockerfile.api # distroless + multi-stage 构建
│ └── Dockerfile.web
│
├── k3s/ # [必须] 生产编排 (单二进制 K8s)
│ │ # 职责: 替代 full K8s, 内置 containerd/traefik/local-path
│ │ # 阶段: Phase 1 (单体) → Phase 2 (服务拆分)
│ │
│ ├── base/ # 通用清单 (不依赖环境)
│ │ ├── namespace.yaml # app 命名空间
│ │ ├── configmap.yaml # 环境变量注入
│ │ ├── deployment.yaml # 滚动更新策略 (replicas=1 → autoscale)
│ │ ├── service.yaml # ClusterIP / LoadBalancer
│ │ └── ingress.yaml # Traefik 路由规则
│ ├── overlays/ # 环境差异化
│ │ ├── dev/ # 资源限制宽松, 开启 debug
│ │ ├── staging/ # 贴近生产配置
│ │ └── prod/ # 严格资源配额, HPA 配置
│ └── scripts/
│ ├── bootstrap-k3s.sh # 一键安装 k3s + 依赖 (containerd/traefik)
│ └── deploy.sh # kubectl/kustomize 部署入口
│
├── security/ # [必须] 安全栈 (轻量化)
│ │ # 职责: 密钥管理 + 基础策略, 禁止过度工程
│ │
│ ├── sops/ # 密钥加密 (age + .sops.yaml)
│ │ ├── .sops.yaml # 加密规则 (按 env 路径)
│ │ └── secrets.enc.yaml # 加密后的配置模板
│ └── policies/ # [必须] 基础安全策略
│ ├── network-policy.yaml # 禁止跨 namespace 默认访问
│ └── pod-security.yaml # restricted 模式 (非 root, 只读根文件系统)
│
└── .gitkeep
│
├── 🛠️ ops/ · [必须] 运维可执行层 (自动化 + 文档)
│ │ # 设计: 所有运维操作必须通过 just 命令, 禁止手写脚本
│ │ # 禁止: 业务代码依赖 ops/ 脚本 (必须通过 API/配置)
│ │ # 验证: `just --list` 必须列出所有运维命令
│ │
│ ├── migrations/ # [必须] 数据库迁移 (按服务隔离)
│ │ └── runner/ # 迁移执行器 (CLI)
│ │
│ ├── observability/ # [必须] 可观测性配置
│ │ ├── otel/ # OpenTelemetry 配置
│ │ └── vector/ # 日志路由 + 转换
│ │
│ ├── scripts/ # [必须] 运维脚本 (最小化)
│ │ ├── bootstrap/
│ │ │ └── vps.sh.gitkeep # 裸机初始化 (仅安装 just/docker/mise)
│ │ ├── verify-contracts.sh.gitkeep # 契约校验脚本
│ │ └── sync-health-check.sh.gitkeep # 同步健康检查
│ │
│ ├── runbooks/ # [必须] 故障处理手册
│ │ ├── incident-response.md.gitkeep
│ │ └── sync-conflict-resolution.md.gitkeep
│ │
│ └── .gitkeep
│
├── 🌐 tools/web3/ # [必须] Web3 本地测试工具
│ │ # 设计: 本地测试网/中继容器/部署脚本，仅做开发环境辅助
│ │ # 禁止: 包含业务逻辑
│ │
│ ├── anvil.sh # 本地 EVM 测试网
│ ├── nostr-relay-docker.yml # 本地 Nostr 中继
│ ├── ton-local-testnet.sh # TON 测试环境
│ └── .gitkeep
│
├── 📚 docs/ · [必须] 知识资产 (架构决策 + 契约文档)
│ │ # 设计: 所有架构决策必须记录为 ADR, 所有契约必须可渲染
│ │ # 禁止: 口头约定 (必须文档化)
│ │ # 验证: 新成员必须能通过 docs/ 理解架构
│ │
│ ├── adr/ # [必须] Architecture Decision Records
│ │ ├── 001-modular-first-not-monolith.md.gitkeep
│ │ ├── 002-bff-layer-necessity.md.gitkeep
│ │ ├── 003-database-strategy-turso-first.md.gitkeep
│ │ ├── 004-agent-harness-constraints.md.gitkeep
│ │ ├── 005-local-first-storage-strategy.md.gitkeep
│ │ └── .gitkeep
│ │
│ ├── architecture/ # [必须] C4 + 上下文映射
│ │ ├── context/ # System Context 图
│ │ ├── container/ # Container 图 (BFF/Services/Infra)
│ │ ├── component/ # 关键组件序列图
│ │ └── sync-flow/ # 同步流程序列图
│ │
│ ├── contracts/ # [必须] 契约文档源 (渲染用)
│ │ ├── http/ # OpenAPI 文档 (Scalar 渲染)
│ │ └── events/ # AsyncAPI 文档
│ │
│ ├── tenancy-model.md # [必须] 多租户隔离策略详解
│ ├── storage-strategy.md # [必须] 三层存储策略详解 [关键]
│ ├── sync-conflict-guide.md # [必须] 冲突解决策略指南
│ ├── dependency-rules.md # [必须] 依赖方向规则 + 校验方式
│ └── .gitkeep
│
├── 🧪 fixtures/ · [必须] 测试数据 + 种子
│ │ # 设计: 所有测试数据必须版本控制, 禁止硬编码
│ │ # 禁止: 生产数据混入 fixtures
│ │ # 验证: `cargo test` 必须使用 fixtures/ 数据
│ │
│ ├── tenants/ # 多租户测试数据
│ ├── seeds/ # 初始化数据
│ ├── sync-scenarios/ # 同步测试场景
│ └── .gitkeep
│
├── 📋 根文档
│ ├── README.md # [必须] 项目入口 + 快速开始
│ ├── GOAL.md # [必须] 项目目标 + 成功标准
│ ├── AGENTS.md # [必须] AI Agent 协作协议 [关键]
│ ├── CONTRIBUTING.md # [必须] 贡献指南 + 架构约束
│ └── .gitkeep
│
├── .github/workflows/ # [必须] CI/CD 流水线
│ ├── ci-web3.yml # 链上程序编译 + 本地测试网集成测试
│ ├── deploy-contracts.yml # 链上部署/升级 (独立于 K8s)
│ └── .gitkeep
│
└── .gitkeep # [必须] 确保空目录被 Git 追踪

```

---

## 📌 核心依赖规则

```

# 依赖方向 (箭头 = "依赖")

apps/_ → packages/contracts/sdk + packages/ui
servers/_ → packages/core + packages/contracts + services/_ (trait only)
services/_ → packages/core + packages/contracts + packages/adapters (via ports/)
packages/\* → packages/core (仅 core 无依赖)

# 禁止规则

❌ apps/_ → services/_ 或 packages/adapters/_
❌ services/_ → services/_ (必须通过 contracts/events 通信)
❌ packages/_ 包含业务逻辑或具体实现
❌ servers/\* 包含领域业务规则

```

---

## 🔑 关键说明

1. **BFF 位置**：`servers/bff/` 按端聚合视图模型，仅依赖 services trait，不写业务规则
2. **Indexer 合并**：`servers/indexer/` 负责协议事件拉取→转换→写入，无业务逻辑
3. **Trait-Only 依赖**：services 通过 Cargo feature 控制 `trait-only` 导出，确保 BFF 仅依赖抽象
4. **验证命令**：`cargo build -p <module> --features trait-only` 必须独立通过
```
