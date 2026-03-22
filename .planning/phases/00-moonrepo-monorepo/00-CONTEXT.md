# Phase 0: moonrepo 工程化基建 - Context

**Gathered:** 2026-03-22
**Status:** Ready for planning (update)

<domain>
## Phase Boundary

搭建符合 OSS 社区最佳实践的 monorepo 工程骨架。包括：moon 配置、目录结构、Cargo workspace、Tauri v2 + SvelteKit 5 SPA 骨架、CI/CD pipeline、完整的开发工具链（git hooks、commit lint、依赖审计、版本管理、包管理器）。所有后续阶段在此骨架上开发。

</domain>

<decisions>
## Implementation Decisions

### 包管理器
- **D-01:** 使用 bun 作为 JS 包管理器（替代 pnpm）
- **D-02:** moon 配置中 `packageManager` 设为 `bun`
- **D-03:** SvelteKit 项目使用 `bun install` / `bun run`

### 版本管理 (Changesets)
- **D-04:** 使用 Changesets 管理 monorepo 版本
- **D-05:** 所有 crate/app 统一从 `v0.1.0` 起始，同版本号迭代
- **D-06:** Changesets 配置文件位于根目录 `.changeset/`

### Conventional Commits (cocogitto)
- **D-07:** 使用 cocogitto 做 conventional commit 校验 + CHANGELOG 生成
- **D-08:** cocogitto 负责 commit 格式校验和 CHANGELOG，Changesets 负责版本 bump 管理
- **D-09:** 职责边界：cocogitto = commit lint + CHANGELOG generation；Changesets = version bump orchestration + release PR

### Git Hooks (Lefthook)
- **D-10:** 使用 Lefthook 管理 git hooks
- **D-11:** commit-msg hook: cocogitto 校验 conventional commit 格式
- **D-12:** pre-commit hook: 运行全部质量门 — `cargo fmt --check` + `cargo clippy` + `bun run lint`
- **D-13:** pre-push hook: 运行 `cargo test` + `bun run test`（快速测试）

### 依赖审计 (cargo-deny)
- **D-14:** 使用 cargo-deny 检查安全漏洞 + 许可证合规
- **D-15:** 许可证白名单：MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-DFS-2016, CC0-1.0
- **D-16:** CI 中 cargo-deny 作为必须通过的 gate

### 依赖更新 (Renovate)
- **D-17:** 使用 Renovate 自动创建依赖更新 PR
- **D-18:** 所有依赖更新走 PR，手动合并（不自动合并）

### SvelteKit 骨架
- **D-19:** 带路由占位页面：Home、TopK、Subscriptions、Resources（对应 Phase 1-4 导航结构）
- **D-20:** 基础布局 shell：sidebar 导航 + 主内容区
- **D-21:** SPA 模式：`ssr=false` + `prerender=false` + `fallback: 'index.html'`

### Crate 内部结构（agent 决定）
- **D-22:** 使用 flat 风格：`src/lib.rs` 作为入口，按职责用 `mod` 声明子模块
- **D-23:** 不使用 `mod.rs` 风格，用 `src/module.rs` 方式（Rust 2018+ 推荐）

### moon tasks（agent 决定）
- **D-24:** 基础任务集：build, test, lint, fmt, check, dev
- **D-25:** 质量门任务：`ci`（聚合 fmt + clippy + test + cargo-deny）
- **D-26:** dev 任务：启动 Tauri dev 模式（`moon run desktop-ui:dev`）

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### 架构规范
- `docs/03_system_architecture_spec.md` — Rust 四层架构、crate 划分、依赖方向约束
- `CLAUDE.md` — 项目技术栈速查、开发命令

### moonrepo
- moonrepo.dev docs — workspace.yml / toolchains.yml 配置参考（research 中已摘录）

### Tauri v2 + SvelteKit
- v2.tauri.app/start/frontend/sveltekit — 官方 SPA 模式配置指南（research 中已摘录）

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- 无（Phase 0 是从零搭建，无现有代码）

### Established Patterns
- Rust 四层架构：Presentation → Application → Domain → Infrastructure（依赖单向）
- SvelteKit 5 Runes：`$state()`, `$derived()`, `$effect()`, `$props()`
- Tauri v2 IPC：`invoke()` 命令调用 Rust 后端

### Integration Points
- `apps/desktop-ui/src-tauri/` ← Tauri Rust 后端入口
- `apps/desktop-ui/src/` ← SvelteKit 前端入口
- `crates/runtime_tauri/` ← Tauri commands 注册点
- 根 `Cargo.toml` ← workspace 成员声明
- `.moon/workspace.yml` ← 项目注册 + 任务继承

</code_context>

<specifics>
## Specific Ideas

### OSS 社区准备
- 项目目标面向 OSS 社区，所有工具链选择需考虑社区友好性
- LICENSE 文件（MIT 或 Apache-2.0）必须在 Phase 0 创建
- CONTRIBUTING.md 可在 Phase 0 或 Phase 5 创建
- README.md 必须在 Phase 0 创建（项目说明 + 快速开始）

### cocogitto + Changesets 协作流程
1. 开发者写 conventional commit（`feat:`, `fix:`, `chore:` 等）
2. Lefthook commit-msg hook 调用 `cog check` 校验格式
3. Release 时：`cog changelog` 生成 CHANGELOG → Changesets bump 版本 → `cog commit` 记录版本变更

### bun workspace 配置
- `package.json` workspaces: `["apps/*"]`
- bun 原生支持 workspace，无需额外配置
- moon 配置 `packageManager: 'bun'`

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 00-moonrepo-monorepo*
*Context gathered: 2026-03-22*
