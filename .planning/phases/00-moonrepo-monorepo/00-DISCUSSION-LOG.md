# Phase 0: moonrepo 工程化基建 - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-22
**Phase:** 00-moonrepo-monorepo
**Areas discussed:** OSS 工具链, 版本管理, SvelteKit 骨架, 包管理器

---

## OSS 工具链选择

| Tool | Role | Selected |
|------|------|----------|
| moon | Task runner + cache | ✓ |
| bun | JS 包管理器 | ✓ |
| Changesets | 版本管理 | ✓ |
| Lefthook | Git hooks | ✓ |
| cocogitto | Conventional commits + CHANGELOG | ✓ |
| Renovate | 依赖更新 PR | ✓ |
| cargo-deny | Rust 依赖审计 | ✓ |
| Tauri official | Build/distribute | ✓ |

**User's choice:** 全部加入，面向 OSS 社区做准备

---

## 包管理器

| Option | Description | Selected |
|--------|-------------|----------|
| bun | 更快、原生 workspace 支持 | ✓ |
| pnpm | 现有研究基于 pnpm | |
| yarn | yarn berry 现代版 | |

**User's choice:** bun（推荐）

---

## Changesets 版本策略

| Option | Description | Selected |
|--------|-------------|----------|
| 统一 v0.1.0 | 所有 crate/app 同版本号迭代 | ✓ |
| 分层起始版本 | 公共 crate 先发，应用 crate 后发 | |

**User's choice:** 统一 v0.1.0

---

## cocogitto + Changesets 职责边界

| Option | Description | Selected |
|--------|-------------|----------|
| 协作模式 | cocogitto = commit lint + CHANGELOG, Changesets = version bump | ✓ |
| cocogitto 仅 commit lint | 版本管理全由 Changesets 手动处理 | |

**User's choice:** 协作模式

---

## Lefthook hooks 范围

| Option | Description | Selected |
|--------|-------------|----------|
| 仅 commit lint | commit-msg hook only | |
| commit lint + 质量门 | commit-msg + pre-commit | |
| 全部（含测试） | commit-msg + pre-commit + pre-push | ✓ |

**User's choice:** 全部（含测试）

---

## cargo-deny 检查范围

| Option | Description | Selected |
|--------|-------------|----------|
| 安全 + 许可证 | 安全漏洞 + 许可证合规（推荐） | ✓ |
| 仅安全 | 仅安全漏洞 | |
| 全量 | 安全 + 许可证 + 重复依赖 | |

**User's choice:** 安全 + 许可证（推荐）

---

## Renovate 策略

| Option | Description | Selected |
|--------|-------------|----------|
| 全部 PR | 所有依赖更新自动创建 PR，手动合并 | ✓ |
| patch 自动合并 | patch 自动，minor/major 需 PR | |
| 保守模式 | 骨架配置，策略后续调 | |

**User's choice:** 全部 PR（推荐）

---

## SvelteKit 骨架内容

| Option | Description | Selected |
|--------|-------------|----------|
| 带路由占位 | Home/TopK/Subscriptions/Resources + 基础布局 | ✓ |
| 纯空白骨架 | 仅 SPA 配置 | |

**User's choice:** 带路由占位（推荐）

---

## Agent's Discretion

| Area | Decision |
|------|----------|
| Crate 内部结构 | flat 风格 `src/lib.rs` + `mod` 声明，不用 `mod.rs` |
| moon tasks 粒度 | build/test/lint/fmt/check/dev + ci 聚合任务 |

---

*Phase: 00-moonrepo-monorepo*
*Discussion date: 2026-03-22*
