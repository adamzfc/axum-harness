# 📁 Project Directory Architecture

> A **Rust-first · Go-fallback** microservice boilerplate for `tauri-sveltekit-axum-moon-template`. Clone via GitHub Template. Ship as a containerized monolith from day one.

---

## 📊 Rust 替代方案可行性矩阵

| 原方案 (Go/通用) | Rust 替代候选 | 成熟度 | 生产可用 | 迁移成本 | 建议 | 架构位置 |
|-----------------|------------|--------|----------|----------|------|----------|
| **Docker Engine** | Youki | ⭐⭐⭐⭐ | ✅ 边缘/测试 | 中 | ⚠️ 观察 | `infra/docker/` |
| | Railcar | ⭐⭐ | ❌ 已归档 | - | ❌ 不推荐 | - |
| **Kubernetes** | Nanocl | ⭐⭐⭐ | ⚠️ 小集群/边缘 | 高 | ⚠️ 边缘场景备用 | `infra/orchestrator/` |
| | K8s (Rust operator) | ⭐⭐⭐⭐⭐ | ✅ 主流 | 低 | ✅ 继续用 + Rust 扩展 | `infra/k8s/` |
| **Consul** | Corrosion (Fly) | ⭐⭐ | ❌ 文档少 | 高 | ❌ 暂不替代 | - |
| | etcd + rust-client | ⭐⭐⭐⭐⭐ | ✅ 成熟 | 低 | ✅ 保留 + Rust SDK | `packages/shared/config/` |
| **Prometheus+Grafana** | OpenObserve | ⭐⭐⭐⭐ | ✅ 生产可用 | 中 | ✅ 成本敏感场景首选 | `ops/observability/` |
| **Log Aggregator** | Vector | ⭐⭐⭐⭐⭐ | ✅ 原生Rust | 低 | ✅ 直接使用 | `ops/observability/vector/` |
| | WarpParse | ⭐⭐ | ❌ 未开源/不明 | - | ❌ 暂不评估 | - |
| **OPA** | Regorus | ⭐⭐⭐⭐ | ✅ 嵌入式场景 | 低 | ✅ 策略引擎嵌入首选 | `infra/security/regorus/` |
| **Kopia/Restic** | Rustic | ⭐⭐⭐⭐ | ✅ restic兼容 | 低 | ✅ 备份工具替代 | `ops/backup/rustic/` |
| | Vykar | ⭐⭐ | ❌ 项目不明 | - | ❌ 暂不评估 | - |
| **核心中间件** | async-nats / rdkafka / sqlx / redis-rs / opentelemetry-rust | ⭐⭐⭐⭐⭐ | ✅ 全成熟 | 低 | ✅ 全面采用 | `services/*/Cargo.toml` |
| **开发工具链** | just / cargo-deny / cargo-tarpaulin / drill | ⭐⭐⭐⭐⭐ | ✅ 原生Rust | 无 | ✅ 强烈推荐 | `tools/` |

> 📌 **结论**：  
> - ✅ **立即可用**：Vector, OpenObserve, Regorus, Rustic, Rust核心库, 开发工具链  
> - ⚠️ **观察/边缘场景**：Youki, Nanocl  
> - ❌ **暂不替代**：Kubernetes(用Rust写Operator扩展), Consul(用etcd+Rust客户端)

---

## 📁 最终目录树架构（带可行性注释）

```
tauri-sveltekit-axum-moon-template/
├── 📦 apps/                          # 【应用层】用户入口 · 技术栈无关
│   ├── 🖥️  client/                   # 客户端应用
│   │   ├── 🌐  web/                  # ✅ SvelteKit 5 + Tailwind · 前端展示层
│   │   ├── 💻  native/               # ✅ Tauri v2 + SvelteKit · 桌面端
│   │   │   └── src-tauri/            # Tauri Rust 后端 (commands/plugins)
│   │   ├── 📱  browser-extension/    # ⚠️ 预留 · 浏览器扩展
│   │   └── 📲  desktop/              # ⚠️ 预留 · 独立桌面端
│   │
│   ├── 🚪  ops/                      # 运维工具应用
│   │   ├── 📚  docs-site/            # ⚠️ 文档站点 (stub)
│   │   └── 🎨  storybook/            # ⚠️ UI 组件文档 (stub)
│   │
│   └── 🔀  bff/                      # 🆕 后端聚合层 · 契约驱动
│       ├── 📱  mobile-bff/           # 🆕 移动端聚合 (Axum) · 按需启用
│       └── 💻  admin-bff/            # 🆕 后台聚合 (Axum) · 按需启用
│
├── 🔧 servers/                       # 【服务层】领域服务 · 组合层
│   ├── 🔌  api/                      # ✅ 现有 Axum API 服务 (保留)
│   │   ├── src/
│   │   │   ├── routes/               # HTTP 路由 (控制器)
│   │   │   ├── middleware/           # 中间件 (认证/限流/追踪)
│   │   │   ├── state.rs              # 应用状态 (连接池/缓存)
│   │   │   ├── error.rs              # 统一错误处理
│   │   │   └── config.rs             # 服务配置
│   │   ├── tests/
│   │   │   ├── e2e/                  # 端到端测试
│   │   │   ├── integration/          # 集成测试
│   │   │   └── ui/                   # UI 契约测试
│   │   └── Cargo.toml                # 独立 crate · 可单独 build
│   │
│   ├── 🚪  gateway/                  # ⚠️ 预留 · API 网关 (stub)
│   │   └→ 未来: Pingora(Rust) · TLS/限流/XDP
│   └── ⚡  realtime/                 # ⚠️ 预留 · 实时服务 (stub)
│       └→ 未来: WebSocket/SSE · Chat 页面推送
│
├── 🧱 packages/                      # 【共享层】跨端复用 · 纯逻辑/契约
│   ├── 🧩  adapters/                 # 适配器层 · 外部世界翻译层
│   │   ├── auth/                     # 认证适配器
│   │   │   ├── google/               # ✅ Google OAuth
│   │   │   ├── oauth/                # ⚠️ 通用 OAuth (stub)
│   │   │   ├── dpop/                 # ⚠️ DPoP (stub)
│   │   │   └── passkey/              # ⚠️ WebAuthn/Passkey (stub)
│   │   ├── hosts/                    # 宿主环境适配器
│   │   │   ├── tauri/                # ✅ Tauri 命令适配
│   │   │   ├── base-app/             # ⚠️ 基础应用 (stub)
│   │   │   ├── browser-extension/    # ⚠️ 浏览器扩展 (stub)
│   │   │   ├── telegram-miniapp/     # ⚠️ Telegram MiniApp (stub)
│   │   │   └── farcaster-miniapp/    # ⚠️ Farcaster MiniApp (stub)
│   │   ├── storage/                  # 存储适配器
│   │   │   ├── surrealdb/            # ✅ SurrealDB 实现
│   │   │   ├── turso/                # ✅ Turso/libSQL 实现
│   │   │   ├── sqlite/               # ⚠️ 原生 SQLite (stub)
│   │   │   ├── indexeddb/            # ⚠️ 浏览器 IndexedDB (stub)
│   │   │   ├── extension-storage/    # ⚠️ 扩展存储 (stub)
│   │   │   └── tauri-store/          # ⚠️ Tauri 文件系统存储 (stub)
│   │   ├── chains/                   # ⚠️ 区块链适配器 (全部 stub)
│   │   ├── protocols/                # ⚠️ 社交协议适配 (全部 stub)
│   │   └── telemetry/                # 🆕 遥测适配器
│   │       ├── otel/                 # OpenTelemetry 集成
│   │       └── tracing/              # tracing 日志集成
│   │
│   ├── 📐  contracts/                # 契约定义层 · 类型唯一真理源
│   │   ├── api/                      # ✅ API DTO 契约 (utoipa + ts-rs)
│   │   ├── auth/                     # ✅ Auth 契约 (登录/注册/Token)
│   │   ├── events/                   # ✅ Events 契约 (领域事件)
│   │   ├── errors/                   # 🆕 统一错误契约 (thiserror 映射)
│   │   ├── ui/                       # ⚠️ UI 组件契约 (stub)
│   │   ├── protocols/                # ⚠️ 协议消息契约 (stub)
│   │   ├── codegen/                  # 🆕 代码生成配置 (ts-rs/prost/utoipa)
│   │   └── generated/                # 自动生成输出 (git-ignored)
│   │
│   ├── 🎯  core/                     # 核心抽象层 · 纯 Rust trait
│   │   ├── domain/                   # ✅ 领域端口定义
│   │   │   └── ports/                # Repository/EventBus trait
│   │   ├── usecases/                 # ✅ 业务逻辑实现 (纯函数 + trait 实现)
│   │   │   ├── admin_service.rs
│   │   │   ├── agent_service.rs
│   │   │   ├── counter_service.rs
│   │   │   └── tenant_service.rs
│   │   └── workspace-hack/           # ✅ cargo-hakari 依赖统一
│   │
│   ├── 🌟  features/                 # 特性定义层 · trait + 类型
│   │   ├── auth/                     # ✅ 认证特性 (登录/注册/会话)
│   │   ├── counter/                  # ✅ 计数器特性 (increment/decrement)
│   │   ├── admin/                    # ⚠️ 管理面板特性 (stub)
│   │   ├── agent/                    # ✅ Agent 特性 (配置/执行/结果)
│   │   ├── chat/                     # 🆕 聊天特性 (消息/会话/推送)
│   │   ├── settings/                 # 🆕 设置特性 (偏好/配置/主题)
│   │   ├── feed/                     # ⚠️ 信息流特性 (stub)
│   │   ├── notifications/            # ⚠️ 通知特性 (stub)
│   │   ├── payments/                 # ⚠️ 支付特性 (stub)
│   │   ├── profile/                  # ⚠️ 用户资料特性 (stub)
│   │   └── social-graph/             # ⚠️ 社交图谱特性 (stub)
│   │
│   ├── 🛠️  shared/                   # 共享技术组件 · 无业务含义
│   │   ├── config/                   # 🆕 多源配置管理 (env/file/etcd)
│   │   ├── errors/                   # 🆕 统一错误处理 (AppError/Result)
│   │   ├── tracing/                  # 🆕 追踪上下文 (OpenTelemetry)
│   │   ├── utils/                    # 🆕 通用工具 (ID生成/加密/时间)
│   │   └── cache/                    # 🆕 缓存抽象 (Moka/Redis 适配)
│   │
│   ├── 🎨  ui/                       # 前端共享 · UI 组件
│   │   └── kit/                      # Svelte 组件库 (按钮/表单/对话框)
│   │
│   └── 🔗  api-contracts/            # 🆕 契约优先 · SDK 生成
│       ├── proto/                    # Protobuf 定义 (gRPC · prost+tonic)
│       ├── openapi/                  # OpenAPI 3.1 聚合 (utoipa 生成)
│       ├── ts-rs-gen/                # Rust→TS 类型同步 (ts-rs)
│       └── sdk-gen/                  # 多语言 SDK 生成 (CI 自动发布)
│
├── 👷  workers/                      # 【后台任务】异步工作者
│   ├── chains/                       # ⚠️ 区块链事件监听 (stub)
│   ├── jobs/                         # ⚠️ 后台任务 (stub)
│   └── protocols/                    # ⚠️ 协议同步 (stub)
│
├── 📡  services/                     # 🆕 【微服务层】未来独立部署
│   │                                 # 每个服务 = 独立 Cargo workspace member
│   ├── 👤  user-service/             # 用户域 · 认证/资料/权限
│   │   ├── src/
│   │   │   ├── domain/               # 【✅ 纯Rust】实体/值对象/事件
│   │   │   ├── application/          # 【✅ 纯Rust】UseCase · 依赖 trait 抽象
│   │   │   ├── interfaces/           # 【✅ Axum/gRPC】HTTP控制器 + DTO
│   │   │   └── infrastructure/       # 【✅ sqlx/redis-rs】DB/Cache实现
│   │   ├── Cargo.toml                # 独立crate · 可单独 build/deploy
│   │   └── openapi.yaml              # 【✅ 契约源】CI自动生成SDK
│   │
│   ├── 🤖  agent-service/            # Agent 域 · 配置/执行/结果
│   ├── 💬  chat-service/             # Chat 域 · 消息/会话/推送 (WebSocket)
│   ├── 📊  counter-service/          # Counter 域 · 计数/统计
│   ├── 🔔  notification-service/     # 通知域 · 推送/模板/渠道
│   └── 📢  event-bus/                # 【✅ async-nats】事件总线适配层
│       ├── src/
│       │   ├── ports/                # EventBus trait (依赖倒置)
│       │   ├── adapters/             # NATS/Kafka/in-memory 实现
│       │   └── outbox/               # 【✅ Outbox Pattern】可靠事件投递
│       └── Cargo.toml                # 共享crate · 被所有服务依赖
│
├── ⚙️  infra/                        # 🆕 【基础设施】声明式运维 · Rust工具链优先
│   ├── 🐳  docker/                   # Docker 开发环境
│   │   ├── compose.dev.yml           # 开发环境 (Postgres/Redis/NATS)
│   │   ├── compose.prod.yml          # 生产环境 (集群模式)
│   │   └── youki/                    # ⚠️ 观察: Youki runtime 配置
│   │
│   ├── ☸️  k8s/                      # Kubernetes 生产编排 · Rust Operator扩展
│   │   ├── base/                     # 通用配置 (Deployment/Service)
│   │   ├── overlays/                 # 环境差异化 (dev/staging/prod)
│   │   ├── istio/                    # 【✅ Istio】Service Mesh · mTLS/金丝雀
│   │   └── nanocl/                   # ⚠️ 边缘备用: Nanocl 配置
│   │
│   ├── 🌍  terraform/                # 云资源编排
│   │   ├── modules/                  # RDS/Redis/NATS/K8s 可复用模块
│   │   └── environments/             # dev/prod 多环境实例
│   │
│   └── 🔐  security/                 # 安全栈
│       ├── regorus/                  # 【✅ Regorus】OPA策略引擎 · Rust嵌入
│       ├── casbin-rs/                # 【✅ casbin-rs】RBAC/ABAC · 内存策略
│       ├── crowdsec/                 # 【✅ CrowdSec(Rust)】威胁情报 + 自动封禁
│       └── lego/                     # 【✅ lego】ACME证书自动续签 · Rust CLI
│
├── 🛠️  ops/                          # 🆕 【运维层】可执行流程 · Rust工具优先
│   ├── 🗄️  migrations/               # 数据库迁移 · 按服务隔离
│   │   ├── api/                      # API 服务迁移
│   │   ├── user-service/             # 用户服务迁移
│   │   └── chat-service/             # 聊天服务迁移
│   │
│   ├── 📊  observability/            # 可观测栈
│   │   ├── openobserve/              # 【✅ OpenObserve】统一日志/指标/追踪
│   │   ├── vector/                   # 【✅ Vector】日志路由 + 转换 · Rust原生
│   │   ├── prometheus/               # 【✅ 兼容】业务指标定义 (备用)
│   │   └── grafana/                  # 【✅ 兼容】预置Dashboard (备用)
│   │
│   ├── 🚀  deploy/                   # 声明式部署
│   │   ├── canary.sh                 # Istio + Prometheus SLO 金丝雀发布
│   │   ├── rollback.sh               # Helm history + K8s 一键回滚
│   │   └── health-check.sh           # 多端健康检查 (DB/Cache/MQ/HTTP)
│   │
│   ├── 🧪  testing/                  # 专项测试
│   │   ├── contract/                 # 【✅ pact-rust】契约测试 · 确保接口兼容
│   │   ├── load/                     # 【✅ drill/lumen】Rust压测 · 高并发场景
│   │   └── chaos/                    # 【✅ chaos-mesh】故障注入 · K8s集成
│   │
│   └── 💾  backup/                   # 备份
│       ├── rustic/                   # 【✅ Rustic】restic兼容 · 加密增量备份
│       └── kopia/                    # 【✅ 兼容备用】大文件/冷备场景
│
├── 📚  docs/                         # 🆕 【知识层】架构资产
│   ├── 🗺️  architecture/             # ADR + 上下文映射
│   │   ├── ADR-001-rust-first.md     # 为什么Rust优先 + Go替补场景
│   │   ├── ADR-002-modular-monolith.md # 模块化单体演进策略
│   │   ├── ADR-003-stun-command.md   # stun 命令设计与使用指南
│   │   └── context-maps/             # DDD上下文映射图 (Mermaid)
│   │
│   ├── 🔌  api/                      # 接口文档源
│   │   ├── openapi/                  # 聚合所有服务的OpenAPI
│   │   └── graphql/                  # BFF层GraphQL Schema
│   │
│   └── 🧭  runbooks/                 # 运维手册
│       ├── incident-response.md      # 故障处理流程
│       └── scaling-guide.md          # 弹性扩缩容指南
│
├── 🛠️  tools/                        # 【工程层】Rust原生工具链
│   ├── 🎯  just/                     # just 任务模块 (已在 justfiles/)
│   ├── 🔍  quality/                  # 代码质量配置
│   │   ├── cargo-deny.ron            # 依赖许可证/安全审计 · CI强制
│   │   ├── clippy.toml               # Rust Lint规则 · 严格模式
│   │   └── golangci.yaml             # Go Lint规则 · 替补模块专用
│   ├── 🧪  testing/                  # 测试基础设施
│   │   ├── testcontainers/           # 【✅ testcontainers-rs】集成测试容器
│   │   └── mock-server/              # 【✅ mockito】外部依赖模拟
│   │
│   ├── 🤖  codegen/                  # 代码生成
│   │   ├── ts-rs.config.ts           # Rust→TS 类型同步 · 前端类型安全
│   │   ├── prost.config.rs           # Protobuf→Rust/Go · gRPC SDK生成
│   │   └── openapi-gen.yaml          # OpenAPI→SDK · 自动发布
│   │
│   ├── evals/                        # ⚠️ 评估 (stub)
│   ├── generators/                   # ⚠️ 生成器 (stub)
│   ├── mcp/                          # ⚠️ MCP 工具 (stub)
│   └── scripts/                      # ⚠️ 脚本 (stub)
│
├── 🧪  e2e-desktop-playwright/       # ✅ 桌面端 E2E 测试
├── 📜  scripts/                      # ✅ TypeScript 构建脚本
├── 📋  justfiles/                    # ✅ Just 命令模块
├── 📄  .github/workflows/            # ✅ CI/CD
├── 🎯  .agents/                      # ✅ AI Agent 技能
├── 🔧  .cargo/                       # ✅ Cargo 配置
├── ⚙️  .config/                      # ✅ hakari/nextest 配置
├── 📋  .planning/                    # ✅ 项目规划
├── 📦  Cargo.toml                    # ✅ Rust workspace
├── 📦  Cargo.lock
├── 📦  package.json                  # ✅ Bun 包管理
├── 📦  package-lock.json
├── 📋  Justfile                      # ✅ 命令入口
├── 🌙  moon.yml                      # ✅ moon 任务
├── 📋  AGENTS.md                     # ✅ AI 协作协议
├── 🎯  GOAL.md                       # 🆕 项目目标文档
├── 📐  DESIGN.md                     # ⚠️ 设计文档 (待填充)
├── 📋  clippy.toml
├── 📋  deny.toml
├── 📋  rustfmt.toml
├── 📋  rust-toolchain.toml
├── 🔧  .tool-versions
├── 🔧  .mise.toml
├── 🌍  .env.example
├── 📋  .gitignore
└── 📋  .gitattributes
```

---

## 🔑 关键设计原则（一句话原则）

| 目录 | 核心原则 | 为什么这样设计 |
|------|----------|----------------|
| `packages/contracts/` | 契约唯一源 · CI自动生成多语言SDK | 前端/其他服务只依赖契约，不依赖实现，后端重构不影响调用方 |
| `packages/core/` | 定义 trait · 基础设施层实现 | 业务逻辑可纯单元测试，数据库/缓存/消息队列可热替换（如 SQLite → TiDB） |
| `packages/adapters/` | 薄翻译层 · 无业务逻辑 | 外部协议变更只影响适配器，核心逻辑零感知 |
| `packages/features/` | 定义 trait + 类型 · 不包含实现 | 特性开关 + 接口契约，未来按特性拆分服务 |
| `servers/api/` | 组合层 · 无业务逻辑 | 路由 → UseCase → 适配器，职责单一可测试 |
| `services/*/` | 每个服务 = 独立 Cargo member + 独立 DB Schema | 未来拆微服务只需 `cargo build -p xxx` + 独立部署，零业务代码改造 |
| `services/event-bus/` | Outbox Pattern + 适配器模式 | 模块间解耦，拆分服务时只需替换 NATS→Kafka 实现，业务逻辑零感知 |
| `packages/api-contracts/` | proto/openapi/ts-rs 三源合一 · CI生成SDK | 前后端类型安全，接口变更自动编译失败，杜绝运行时错误 |
| `ops/observability/openobserve/` | 统一可观测平台 · 替代 Prometheus+ES+Jaeger | 存储成本降低 50%+，运维复杂度降低，查询语言统一 |
| `infra/security/regorus/` | 嵌入式策略引擎 · Rust原生集成 | 策略校验与业务代码同进程，无网络开销，审计日志自动归集 |
| `tools/` | Rust原生工具链 · 替代Make/Shell | 跨平台一致性，类型安全，缓存友好，新人上手成本降低 |

---

## 🔄 演进路径（模块化单体 → 微服务）

### 阶段 0: 模块化单体 (当前)

```
├─ packages/core/usecases/ 进程内 trait 调用
├─ services/ 目录预留，逻辑在 packages/
├─ event-bus 用 in-memory channel + Outbox 表
├─ 数据库: Turso/SurrealDB 单实例
├─ 部署: 单个 binary + docker-compose
└─ 可观测: Vector → OpenObserve 单节点
```

**触发条件**: 服务变更频率 > 3次/周 或 团队 > 5人

       ↓

### 阶段 1: 独立构建 (准备拆分)

```
├─ 每个服务可单独 `cargo build -p xxx-service`
├─ event-bus 适配 async-nats (dev用docker-compose启动)
├─ 数据库: 按服务分 schema + 禁止跨schema JOIN
├─ 部署: 每个服务独立 Docker 镜像 + 同一 K8s Deployment
└─ 可观测: OpenObserve 集群模式 + 多租户隔离
```

**触发条件**: 服务需独立扩缩容 或 技术栈差异化

       ↓

### 阶段 2: 独立部署 (微服务化)

```
├─ 服务间通信: in-memory → gRPC (via Istio Sidecar)
├─ event-bus: dev NATS → prod Kafka Cluster
├─ 数据库: 物理隔离 (RDS实例/独立连接池)
├─ 部署: 每个服务独立 K8s Deployment + HPA + Istio
├─ 安全: Regorus 策略引擎嵌入每个服务 + CrowdSec 实时防护
└─ 可观测: OpenObserve + Vector 多节点 + 链路追踪
```

       ↓

### 阶段 3: 边缘/混合云 (可选)

```
├─ 边缘节点: Nanocl 替代 K8s + Youki 替代 runc
├─ 服务发现: etcd + rust-client 替代 Consul
├─ 备份: Rustic 加密增量备份 + 冷热分层
└─ 部署: GitOps (ArgoCD) + 多区域容灾 + 自动故障转移
```

---

## 🎯 Command Surface: Just + xx.just Only

### Philosophy

**Shell scripts are limited to bare VPS bootstrapping.** Everything else — dev, test, build, deploy, migrate — is driven by `just` and `justfiles/*.just` modules.

```
scripts/bootstrap/
└── vps.sh                    # ONLY shell script — installs just, docker, git, and essential ops CLI
                                # (jq, yq, rg, fd, mise, etc.)

justfiles/                      # ALL operations live here
├── setup.just                  # Toolchain installation, doctor, sccache, hakari
├── dev.just                   # Dev servers (fullstack, web, API, desktop, Tauri)
├── test.just                  # Unit, integration, contract, E2E, coverage
├── quality.just               # Format, lint, boundary-check, verify
├── build.just                 # Workspace build, single-service build, cross-compile
├── deploy.just                # Docker Compose deploy, systemd deploy, K8s deploy
├── migrate.just               # DB migrations up/down, status, rollback
├── processes.just             # Cross-platform process management (ps, stop, ports)
├── clean.just                 # cargo clean, sweep, coverage cleanup
└── skills.just                # Agent skills integration
```

### VPS Bootstrap Script (`scripts/bootstrap/vps.sh`)

This is the **only** shell script in the project. It sets up a bare VPS with the minimum required tools:

```bash
#!/usr/bin/env bash
# scripts/bootstrap/vps.sh
# Purpose: Bootstrap a bare VPS with essential tools
# Idempotent: Safe to run multiple times

set -euo pipefail

# 1. Install just (cross-platform command runner)
if ! command -v just &>/dev/null; then
  curl -sSf https://github.com/casey/just/releases/latest/download/just-x86_64-unknown-linux-musl.tar.gz | tar -xz -C /usr/local/bin/ just
  chmod +x /usr/local/bin/just
fi

# 2. Install Docker
if ! command -v docker &>/dev/null; then
  curl -fsSL https://get.docker.com | sh
  usermod -aG docker "${USER}"
fi

# 3. Install essential ops CLI tools
if ! command -v mise &>/dev/null; then
  curl -sSf https://github.com/jdx/mise/releases/latest/download/mise-latest-linux-x64.tar.gz | tar -xz -C /usr/local/bin/ mise
  chmod +x /usr/local/bin/mise
fi

# ... (jq, yq, rg, fd, etc.)

echo "✅ VPS bootstrapped. Run: just setup"
```

### Just Command Surface

```bash
# === Setup ===
just setup                      # Install all toolchains (Rust, Bun, just, mise, etc.)
just doctor                     # Verify all dependencies are installed

# === Development ===
just dev up                     # Start full dev environment (web + API + DB)
just dev web                    # Start SvelteKit dev server only
just dev api                    # Start Axum API server only
just dev tauri                  # Start Tauri desktop app
just dev desktop                # Start desktop client

# === Testing ===
just test unit                  # Run unit tests
just test integration           # Run integration tests
just test e2e                   # Run E2E tests (Playwright)
just test contract              # Run contract tests (pact-rust)
just test coverage              # Run tests with coverage
just test load                  # Run load tests (drill)

# === Quality ===
just quality                    # Run all quality checks (format + lint + deny + boundary)
just quality format             # Format all code
just quality lint               # Run clippy + biome
just quality deny               # Run cargo-deny
just quality boundary           # Check architecture boundary violations

# === Build ===
just build                      # Build entire workspace
just build -p user-service      # Build single service
just build --release            # Release build
just build --target x86_64-unknown-linux-musl  # Cross-compile

# === Deploy ===
just deploy compose             # Deploy via Docker Compose
just deploy systemd             # Deploy to systemd (Linux server)
just deploy k8s                 # Deploy to Kubernetes
just deploy canary              # Canary deployment (Istio)
just deploy rollback            # Rollback last deployment

# === Migrate ===
just migrate up                 # Run all pending migrations
just migrate down               # Rollback last migration
just migrate status             # Show migration status
just migrate create add_users_table  # Create new migration

# === Process Management ===
just ps                         # Show running processes
just ps stop                    # Stop all processes
just ps ports                   # Show listening ports

# === Cleanup ===
just clean                      # Clean all build artifacts
just clean cargo                # cargo clean
just clean coverage             # Clean coverage reports
```

### Why Just, Not Shell Scripts?

| Aspect | Shell Scripts (.sh) | Just + xx.just |
|--------|-------------------|----------------|
| **Cross-platform** | ❌ Platform-specific, needs bash/zsh/PowerShell variants | ✅ Works on macOS/Linux/Windows (WSL) |
| **Type safety** | ❌ Stringly-typed, no validation | ✅ Justfile syntax is validated |
| **Discoverability** | ❌ Must read source or `--help` | ✅ `just --list` shows all commands |
| **Dependencies** | ❌ Manual dependency management | ✅ Just handles recipe dependencies |
| **Caching** | ❌ Always re-runs | ✅ Can integrate with sccache, cargo caching |
| **Composability** | ❌ Hard to chain scripts | ✅ Recipe composition, variable passing |
| **Agent-friendly** | ❌ AI agents struggle with shell nuances | ✅ Structured, predictable syntax |

**Rule of thumb**: If it needs to run on a bare VPS before anything else exists, it's a `.sh` script. Everything else is `just`.

---

## 🎯 Acceptance Criteria (How to Know the Architecture Succeeds)

1. **Painless Split**: Extracting `user-service` into its own repo requires only `Cargo.toml` path + deploy config changes, zero business logic modification
2. **Frontend Agnostic**: BFF layer aggregates interfaces; frontend depends only on `api-contracts/sdk-gen/typescript`; backend topology changes don't affect frontend
3. **Ops Control**: `just deploy compose` one-command deploy; `ops/observability/openobserve/` pre-configured business dashboard
4. **Security Compliance**: `cargo-deny` + `crowdsec` + `regorus` three-layer protection; audit logs auto-collected
5. **Efficiency**: `just build -p user-service` compiles only changed modules; CI < 5 min; newcomer `just dev up` starts full stack in 3 min
6. **Cost Optimization**: OpenObserve replaces Prometheus+ES+Jaeger; storage cost reduced 50%+; unified query language
7. **Template Ready**: GitHub "Use this template" → `git clone` → `just setup` → `just dev up` → running app. Newcomer needs zero architecture design knowledge to start developing

---

> 💡 **Core Philosophy**:
> **Use Rust's type system and ownership model to gain determinism and safety in microservice evolution**.
> Spend 1 hour today defining `Repository trait`, save 100 hours tomorrow refactoring business logic;
> Spend 1 hour today configuring `OpenObserve`, save 100 hours maintaining multiple monitoring systems.
> **Design discipline + Rust ecosystem = evolution freedom + ops cost reduction**.
