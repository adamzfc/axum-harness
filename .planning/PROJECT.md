# Tauri-SvelteKit-Axum Boilerplate

## What This Is

A production-ready boilerplate/template for building cross-platform desktop applications using Tauri 2 + SvelteKit + Axum + moon workflow. Target: developers who want a fully configured starting point with best practices for medium-scale projects.

## Core Value

Provide a runnable, tested, production-ready boilerplate with authentication (Google OAuth), multi-tenancy, backend infrastructure (containerized Redis/cache, database, reverse proxy), and full stack best practices — so developers can start building business logic immediately.

## Current Milestone: v0.1.1 架构收敛、决策沉淀与生产闭环

**Goal:** 在最小改动原则下，把关键架构建议全部落盘为可追踪决策，并完成高优先级生产闭环能力，确保后续 agent 迭代稳定且低认知负担。

**Target features:**
- 安全基线硬化（JWT 校验链路、敏感配置治理、路径可移植性）
- 契约与类型闭环（`contracts_api` + Rust/TS typegen 自动同步）
- 运行时边界收敛（`runtime_tauri` 落地，native host 瘦身）
- Moon/Just 任务体系补全（`fullstack:dev`、`typegen`、`verify`）
- 全量建议决策账本（逐条记录是否实现、何时实现、如何实现、原因）
- 后续 phase 简要概括（即使不在本里程碑实施也不丢失）

## Requirements

### Validated

- ✓ Tauri 2 desktop app scaffolding — existing
- ✓ SvelteKit frontend foundation — existing
- ✓ Axum backend server — existing
- ✓ moon build toolchain — existing
- ✓ Mobile-first responsive layout base — existing
- ✓ Frontend dependencies aligned with TECH_SELECTION.md — Validated in Phase 01: package-foundation
- ✓ Rust workspace dependencies pinned with release profile — Validated in Phase 01: package-foundation
- ✓ All 7 Tauri plugins registered — Validated in Phase 01: package-foundation
- ✓ moon parallel lint/test configured — Validated in Phase 01: package-foundation
- ✓ Database infrastructure (SurrealDB + libsql dual-DB) — Validated in Phase 05: database-infrastructure
- ✓ Multi-tenant data isolation (tenant_id scoping) — Validated in Phase 07: multi-tenant-data-isolation

### Active

- [ ] 安全基线达到可发布标准（JWT 校验、敏感配置、路径移植）
- [ ] Rust/TS 契约实现自动同步，避免手写类型漂移
- [ ] `runtime_tauri` 与 native host 的职责边界清晰且落地
- [ ] Moon/Just 提供统一的全栈开发与验证入口
- [ ] 历史建议形成决策清单并映射到当前/后续 phase

### Out of Scope

- [Email/password auth] — Google OAuth sufficient for boilerplate
- [Complex RBAC] — Basic multi-tenancy only for v1

## Context

**Current state:** Existing milestone implementation has reached strong scaffold maturity (Axum router/middleware, tenant isolation, E2E layers), but there are strategic gaps blocking long-term low-friction agent iteration: contracts crate is still placeholder, Rust↔TS type pipeline is missing, runtime boundary is drifting toward host code, and several security/portability defaults need hardening.

**Tech stack:**
- Frontend: SvelteKit + bitsUI + TailwindCSS v4 + VitePress + @pqoqubbw/icons + Lottie
- Desktop: Tauri 2.10.3
- Backend: Axum 0.8.8
- Database: SurrealDB (服务端) + libsql/turso (本地 App) - 双数据库架构
- Build: moon
- Testing: Maestro (移动端) + Playwright (Web E2E)

**MCP/Skills needed locally:**
- Code index MCP (for codebase search)
- Websearch MCP
- Research MCP
- Frontend skills (Svelte, Tailwind, bitsUI)
- Backend skills (Axum, Rust)
- Tauri skills
- Testing skills

**UI Requirements:**
- Mobile-first, responsive design
- Three pages: Login, Counter, Admin dashboard
- Platform-agnostic (desktop + mobile web)

**Task lists (template features to configure):**

For package.json: vitepress, lottieplayer, and other utilities to preload but comment out unused

For Cargo (tauri + axum): Deep dive into docs for plugins and dependencies, preload but comment out unused

**Date reference:** March 28, 2026 — verify all versions/dependencies are current

## Constraints

- **[Stack]**: Tauri2 + SvelteKit + Axum + moon — Full-stack Rust/WebView
- **[Timeline]**: Best effort for production-ready quality
- **[Scope]**: Desktop-first but web-accessible, mobile-responsive
- **[Testing]**: Must have passing tests for core flows before release
- **[Infra]**: Docker-compose for local backend (Redis-like cache, database, nginx reverse proxy)

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Dual DB: SurrealDB + libsql/turso | SurrealDB(服务端) + libsql(本地App) 双架构 | ✓ Implemented (Phase 05) |
| Google OAuth only | Reduce boilerplate complexity | — Pending |
| Maestro + Playwright | 移动端用 Maestro, Web用 Playwright | — Pending |
| VitePress (静态) | 构建后纯 HTML, 不占服务器资源 | — Pending |
| release-plz + git-cliff | CI/CD 自动化,综合评估不纯追 Rust | — Pending |
| Fine granularity phases | Maximum flexibility for feature iteration | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---

*Last updated: 2026-04-01 after starting milestone v0.1.1 (architecture convergence and production closure)*
