# 03. 工具链与任务图

> **文档版本**: 1.2.0  
> **最后更新**: 2026-04-01

## 核心原则：工具链解耦

**工具链是手段，不是目的。**

本章节描述的工具选择都是可替代的。关键是通过清晰的职责划分和接口隔离，使得任何工具都可以被替换而不影响系统架构。

### 工具选型决策框架

**我们选择工具的标准：Agent-Friendly、Rust 优先、解决长期痛点。**

| 维度 | 权重 | 说明 |
|------|------|------|
| **Agent 友好** | **最高** | 工具的文档和 API 是否适合 agent 学习和使用 |
| **性能表现** | **高** | 工具的性能是否最优，是否使用 Rust 等高性能语言实现 |
| **Rust 生态** | **高** | 工具是否是 Rust 生态的一部分，是否与 Rust 工具链良好集成 |
| **解决痛点** | **高** | 工具是否解决了传统工具的长期痛点 |
| **社区活跃度** | **中** | 社区是否活跃，是否有持续的维护和发展 |
| **跨平台支持** | **低** | 是否支持目标平台 |

### 选型已确定

以下工具选型已确定，不再频繁变更：

- **任务编排**：moon（确定）
- **包管理器**：Bun（确定）
- **工具链管理**：proto（确定）
- **任务入口**：Just（确定）

## 工具职责划分

### 包管理器 (当前选择: Bun)

**职责**：
- JS/TS runtime
- package manager
- workspace 依赖安装
- 前端测试与轻量脚本执行

**接口契约**：
- 提供 `install`、`test`、`run` 命令
- 支持 workspace 协议
- 支持 lockfile 机制
- 兼容 npm 生态

### 任务编排器 (当前选择: moon)

**职责**：
- monorepo task graph
- project-level tasks
- 任务依赖关系
- workspace 级编排
- CI/CD 标准化入口

**接口契约**：
- 定义任务依赖图
- 支持增量执行
- 提供缓存机制
- 支持远程缓存

### 工具链版本管理 (当前选择: proto)

**职责**：
- 多语言工具链版本管理
- Rust / Bun / Node / 其他 CLI 的版本固定
- 团队环境一致性

**接口契约**：
- 提供版本锁定机制
- 支持自动切换
- 支持多语言管理

### 任务入口 (当前选择: Just)

**职责**：
- 给人类与 agent 的短命令入口
- 收敛常用操作
- 组合 moon / bun / cargo / docker 等命令

**接口契约**：
- 提供简单的命令定义
- 支持参数传递
- 支持依赖执行
- 跨平台兼容

## 角色边界

### 错误做法

- 用 Just 充当复杂 task graph
- 用 moon 充当 package manager
- 用 Bun 管理 Rust 工具链
- 用随机脚本替代可发现任务
- **在业务代码中硬编码工具特定逻辑**

### 正确做法

- `just dev` -> 调 moon 组合任务
- `moon run repo:typegen` -> 调 codegen 链
- `bun test` -> 在 JS/TS 项目局部运行测试
- `proto use` / `.prototools` -> 保证全员同版本工具链
- **通过接口隔离工具实现细节**

## 一级任务建议

### setup
- `repo:setup`
- `repo:bootstrap`
- `repo:doctor`
- `repo:toolchain-check`

### dev
- `repo:dev-web`
- `repo:dev-desktop`
- `repo:dev-extension`
- `repo:dev-api`
- `repo:dev-workers`
- `repo:dev-fullstack`

### quality
- `repo:fmt`
- `repo:lint`
- `repo:typecheck`
- `repo:contracts-check`
- `repo:test-unit`
- `repo:test-integration`
- `repo:test-e2e`
- `repo:test-agent`
- `repo:verify`

### codegen
- `repo:typegen`
- `repo:openapi-gen`
- `repo:fixtures-gen`
- `repo:icons-gen`
- `repo:tokens-gen`

### ops
- `repo:trace-open`
- `repo:evals-run`
- `repo:replay-protocol`
- `repo:release-dry-run`
- `repo:release-desktop`
- `repo:release-web`
- `repo:release-server`

### security
- `repo:audit-rust`
- `repo:audit-bun`
- `repo:secrets-scan`
- `repo:licenses-check`

## 建议命名规范

- repo 级任务：`repo:*`
- package 级任务：`<project>:*`
- feature 测试：`feature-<name>:test`
- adapter 测试：`adapter-<name>:test`

## Just 顶层入口建议

- `just setup`
- `just dev`
- `just verify`
- `just test`
- `just typegen`
- `just release`
- `just evals`

原则：  
Just 只暴露最常用的稳定入口，不承担复杂编排逻辑。

## 必须存在的"doctor"任务

`repo:doctor` 至少检查：

- 包管理器版本（Bun）
- 工具链版本管理器状态（proto）
- Rust toolchain
- 任务编排器版本（moon）
- 环境变量是否齐全
- 本地 services 是否可达
- codegen 产物是否过期
- AGENTS 与规则模板是否存在
