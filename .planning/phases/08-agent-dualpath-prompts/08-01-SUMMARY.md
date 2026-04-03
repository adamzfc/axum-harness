---
phase: 08-agent-dualpath-prompts
plan: 01
subsystem: agent
tags: [tauri, ipc, channel-api, svelte, sse, dual-path]
dependency_graph:
  requires: []
  provides: [AGENT-01]
  affects: [agent page, tauri commands, ipc client]
tech_stack:
  added:
    - feature-agent (workspace dep to runtime_tauri)
    - futures-util (workspace dep to runtime_tauri)
  patterns:
    - Tauri 2 Channel API for streaming
    - AsyncGenerator for frontend streaming
    - Runtime environment detection via window.__TAURI__
key_files:
  created:
    - apps/client/web/app/src/lib/ipc/agent.ts
    - packages/adapters/hosts/tauri/src/commands/agent.rs
  modified:
    - packages/adapters/hosts/tauri/src/commands/mod.rs
    - apps/client/native/src-tauri/src/lib.rs
    - packages/adapters/hosts/tauri/Cargo.toml
    - apps/client/web/app/src/routes/(app)/agent/+page.svelte
decisions:
  - "Channel<String> 使用 owned String 而非 &String（cargo check 发现）"
  - "复用 LibSqlAgentService 而非引入 async-openai，最小依赖"
  - "Tauri 和浏览器路径统一调用 agentChatStream，内部自动路由"
metrics:
  duration: ~5min
  completed_date: "2026-04-03T02:30:00Z"
  tasks_completed: 3
  files_created: 2
  files_modified: 4
---

# Phase 08 Plan 01: Agent 双路径 IPC 实现 Summary

**One-liner:** Agent 页面 Tauri IPC 双路径 — 新建 `lib/ipc/agent.ts` 封装 Channel streaming + SSE fallback，新增 Rust `agent_chat` Tauri command，改造 `agent/+page.svelte` 运行时检测环境并路由。

## Tasks Completed

| # | Task | Commit | Files |
|---|------|--------|-------|
| 1 | 新建 lib/ipc/agent.ts 双路径 client | `c5fa75e` | `apps/client/web/app/src/lib/ipc/agent.ts` |
| 2 | 新增 Rust agent_chat Tauri command | `a0a9db9` | `agent.rs`, `mod.rs`, `lib.rs`, `Cargo.toml` |
| 3 | 改造 agent/+page.svelte 双路径路由 | `7e6e5e0` | `+page.svelte` |

## Implementation Details

### Task 1: lib/ipc/agent.ts 双路径 client

创建 `agentChatStream` async generator 函数：
- **运行时检测**：`(window as { __TAURI__?: unknown }).__TAURI__` 存在性
- **Tauri 路径**：创建 `Channel<string>`，通过 `invoke('agent_chat', { channel })` 调用 Rust 后端，使用 Promise resolve 模式将 channel.onmessage 回调转为 AsyncGenerator yield
- **浏览器路径**：`fetch()` POST 到 `/api/agent/chat`，解析 SSE `data: ` 前缀行，yield content token
- **错误处理**：try/catch 包裹，yield `Error: ${message}` 格式

### Task 2: Rust agent_chat Tauri command

- **agent.rs**：实现 `#[tauri::command] agent_chat`，接收 `Channel<String>` 参数，调用 `LibSqlAgentService::chat_stream()` 获取 Stream，逐 chunk 通过 `channel.send()` 推送
- **mod.rs**：添加 `pub mod agent;`
- **lib.rs**：导入 `agent` 模块，注册 `agent::agent_chat` 到 `invoke_handler`
- **Cargo.toml**：runtime_tauri 添加 `feature-agent` 和 `futures-util` 依赖

### Task 3: agent/+page.svelte 双路径路由

- 导入 `agentChatStream` 从 `$lib/ipc/agent`
- `sendMessage` 函数中添加 `isTauri` 运行时检测
- Tauri 和浏览器路径均调用 `agentChatStream()`，内部自动选择正确路径
- 保留所有现有 UI 模板不变

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Channel<String> send 类型不匹配**
- **Found during:** Task 2 cargo check 验证
- **Issue:** `channel.send(&text)` 和 `channel.send(&format!(...))` 传递 `&String`，但 `Channel<String>::send` 需要 owned `String`
- **Fix:** 移除 borrow，改为 `channel.send(text)` 和 `channel.send(format!(...))`
- **Files modified:** `packages/adapters/hosts/tauri/src/commands/agent.rs`
- **Commit:** `d3f1a2c`

**2. [Rule 2 - Missing dependency] runtime_tauri Cargo.toml 缺少 feature-agent 和 futures-util**
- **Found during:** Task 2 创建 agent.rs 时
- **Issue:** agent.rs 使用了 `feature_agent::AgentService` 和 `futures_util::StreamExt`，但 runtime_tauri/Cargo.toml 未声明这些依赖
- **Fix:** 添加 `feature-agent = { workspace = true }` 和 `futures-util = { workspace = true }`
- **Files modified:** `packages/adapters/hosts/tauri/Cargo.toml`

## Verification Results

| Check | Status |
|-------|--------|
| `cargo check -p runtime_tauri` | ✅ PASS |
| `cargo check -p native-tauri` | ✅ PASS (0 errors) |
| `npx tsc --noEmit` | ✅ PASS (0 errors) |
| agent.ts: Channel + SSE fallback | ✅ PASS |
| agent.rs: #[tauri::command] + channel.send() | ✅ PASS |
| +page.svelte: __TAURI__ + agentChatStream | ✅ PASS |
| mod.rs: pub mod agent | ✅ PASS |
| lib.rs: agent::agent_chat registered | ✅ PASS |

## Known Stubs

None — all functionality is wired end-to-end.

## Self-Check: PASSED

All files exist, all commits verified, all verification checks passed.
