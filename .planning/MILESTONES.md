# MILESTONES: Tauri-SvelteKit-Axum Boilerplate

## v0.2.0 架构蓝图对齐与核心功能实现 (Shipped: 2026-04-04)

**Delivered:** Blueprint-aligned architecture with contracts/typegen闭环, runtime boundary convergence, and minimal feature set (Google Auth, Counter, Admin Web, Agent Chat)

**Phases completed:** 1-8 (24 plans, 46 tasks)

**Key accomplishments:**
- Blueprint-compliant Justfile with 16 commands delegating to moon, replacing old direct-tool-invocation entry points
- Three contracts crates (api, auth, events) with ts-rs derive macros, 8 auto-generated TypeScript DTOs, and moon typegen+drift-check pipeline
- Migrated storage adapters (SurrealDB, LibSQL) into independent crates, enabling hexagonal architecture
- 从 usecases crate 移除 contracts_api 依赖，强制六角形架构边界：core 层只依赖 domain（Port traits）
- Moved Tauri command handlers from native-tauri to runtime_tauri adapter crate; native-tauri reduced to thin bootstrap
- cargo-deny dependency direction rules, CI boundary-check task, and agent-readable boundary compliance rubrics
- CounterService + AdminService implemented as hexagonal feature crates with LibSQL persistence
- End-to-end wiring of counter and admin features across Tauri commands, Axum REST routes, and Svelte frontend
- 交付了基于 LibSQL 持久化与 Axum SSE 流式响应的 Agent Chat 闭环
- 将 runtime_tauri auth 从内联 OAuth 实现重构为 feature-auth + adapter-google 的 host 薄封装调用链
- Agent 页面 Tauri IPC 双路径 + prompts 模板 + Phase 5 VERIFICATION.md
- 前端消费 generated types，消除 inline 重复定义，修复 bigint 兼容性问题

**Stats:**
- 125 files modified
- ~649K lines of code (Rust + TypeScript + Svelte)
- 8 phases, 24 plans, 46 tasks
- 3 days from start to ship (2026-04-01 → 2026-04-03)

**Git range:** `7d82398` → `0b3d24e`

**Known gaps:**
- AUTH-01: GoogleAuthAdapter not fully wired into Tauri commands (Phase 6 empty, tech debt)

**What's next:** Next milestone — host adapter体系做实, offline sync, release automation, tracing/otel

---

## Milestone History

### v0.1.0 — 基础设施搭建与核心功能实现

**Goal:** 从零搭建 Tauri 2 + SvelteKit + Axum + moon 全栈桌面应用模板，实现基础功能闭环。

**Completed:** 2026-04-01

**Delivered features:**

- Package foundation (frontend + Rust workspace deps, 7 Tauri plugins)
- UI styling infrastructure (TailwindCSS v4, dark mode, component system)
- Application pages (Login, Counter, Admin dashboard)
- Backend dependencies & build optimization (Axum middleware stack)
- Database infrastructure (SurrealDB + libsql dual-DB, domain ports)
- Google OAuth authentication
- Multi-tenant data isolation (TenantId, JWT middleware, tenant init API)
- Desktop native features
- Cross-platform build pipeline (GitHub Actions CI matrix)
- Test suite (30 Rust tests, 28 Vitest tests, 28 Playwright E2E tests = 86 total)

**Phases:** 01-10 (archived at `.planning/milestones/v0.1.0-phases/`)

---

*Last updated: 2026-04-04 after v0.2.0 milestone completion*
