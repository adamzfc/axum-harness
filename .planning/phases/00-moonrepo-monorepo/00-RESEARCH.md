# Phase 0 Research: moonrepo 工程化基建

**Date:** 2026-03-22
**Status:** RESEARCH COMPLETE

## moonrepo/moon 关键发现

### 初始化
- `moon init` 创建 `.moon/workspace.yml` + `.moon/toolchains.yml`
- v2 使用 YAML 配置（非 JSON）
- 支持 `--minimal` 快速初始化

### 多语言 workspace 配置
```yaml
# .moon/workspace.yml
projects:
  - 'apps/*'
  - 'crates/*'
vcs:
  client: 'git'
  defaultBranch: 'main'
```

### 工具链管理
```yaml
# .moon/toolchains.yml
rust:
  version: '1.82.0'
node:
  version: '22.11.0'
  packageManager: 'pnpm'
```

### 任务继承
- 根级 `moon.yml` 定义继承任务
- 项目级 `moon.yml` 覆盖/扩展
- 支持 `deps` 声明任务间依赖

### CI 命令
- `moon ci` — 自动检测变更项目并运行
- `moon run :build` — 全局构建
- 支持 `--plan plan.json` 执行计划

## Cargo Workspace

### 结构（对应 docs/03 架构规范 §5）
```
/geek-taste
  /apps
    /desktop-ui           # SvelteKit + Tauri frontend shell
  /crates
    /domain               # 纯领域对象与规则
    /application          # 用例编排
    /github_adapter       # GitHub REST client + mapping
    /persistence_sqlite   # SQLite repository impl
    /notification_adapter # 桌面通知
    /runtime_tauri        # Tauri commands / bootstrap
    /runtime_server       # Axum (future)
    /shared_contracts     # JSON schema / DTO / enum export
```

### workspace Cargo.toml
```toml
[workspace]
members = ["apps/*", "crates/*"]
resolver = "2"

[workspace.dependencies]
# Tauri
tauri = { version = "2", features = [] }
tauri-build = "2"
tauri-plugin-notification = "2"
tauri-plugin-shell = "2"
tauri-plugin-dialog = "2"
tauri-plugin-store = "2"
tauri-plugin-updater = "2"
tauri-plugin-window-state = "2"

# 异步
tokio = { version = "1", features = ["full"] }

# HTTP
reqwest = { version = "0.13", features = ["json", "rustls-tls"] }

# GitHub
octocrab = "0.49"

# SQLite
rusqlite = { version = "0.38", features = ["bundled"] }
rusqlite_migration = "2.4"

# 安全存储
keyring = { version = "3.6", features = ["apple-native", "windows-native"] }

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# 错误
anyhow = "1"
thiserror = "2"

# 日志
tracing = "0.1"
tracing-subscriber = "0.3"

# 时间
chrono = "0.4"
```

**注意:** `rusqlite` 必须启用 `bundled` 特性（避免系统库版本不一致）。
**注意:** `reqwest` 使用 `rustls-tls`（跨平台一致，无需 OpenSSL）。
**注意:** `keyring` 按平台启用特性（apple-native / windows-native / sync-secret-service）。

## Tauri v2 + SvelteKit 5 SPA 配置

### 关键配置（Tauri 官方指南 2026）
1. **svelte.config.js**: 使用 `@sveltejs/adapter-static` + `fallback: 'index.html'`
2. **src/routes/+layout.ts**: `export const ssr = false` + `export const prerender = false`
3. **tauri.conf.json**: `frontendDist: "../build"` + `devUrl: "http://localhost:5173"`
4. **CSP**: Tauri 默认 CSP 配置，注意 hash 模式兼容性

### Svelte 5 Runes
- `$state()` 替代 `let` 声明式响应式
- `$derived()` 替代 `$:` 响应式声明
- `$effect()` 替代 `onMount`/`afterUpdate`
- `$props()` 替代 `export let`
- 不需要外部状态管理库

### SPA 模式要点
- 所有路由由前端 JS 处理
- `+server.js` 端点不可用（无 server runtime）
- `+page.server.js` load 函数不可用
- `+page.js` load 函数在客户端执行

## 依赖兼容性

| 关注点 | 状态 |
|--------|------|
| Tauri v2 + SvelteKit 2 | ✅ 官方支持 |
| tokio 版本对齐 | ⚠️ 确保所有依赖用 tokio 1.x |
| rusqlite bundled | ✅ 推荐启用 |
| reqwest rustls | ✅ 跨平台一致 |
| keyring 跨平台 | ✅ macOS/Windows/Linux |
| Svelte 5 Runes + Tauri IPC | ✅ 无冲突 |

## 风险与缓解

| 风险 | 缓解 |
|------|------|
| moon 对 Rust 项目支持不如 JS 成熟 | 使用 moon 仅做 task runner + cache，Cargo 做实际构建 |
| Tauri v2 ACL 权限配置 | Phase 1 验证，Phase 0 仅搭建骨架 |
| Cargo workspace 成员间循环依赖 | 遵循四层架构，依赖单向（presentation → application → domain） |

## 来源
- moonrepo.dev/docs/setup-workspace
- moonrepo.dev/docs/config/workspace
- v2.tauri.app/start/frontend/sveltekit
- docs/03_system_architecture_spec.md
- CLAUDE.md (project stack)
