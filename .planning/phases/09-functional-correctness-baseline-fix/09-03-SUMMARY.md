---
phase: 09-functional-correctness-baseline-fix
plan: 03
subsystem: client-web-agent
tags: [sveltekit, agent-chat, regression, playwright, vitest]
requires: [AGENT-02, AGENT-03]
provides: [fresh-thread-reset, settings-retention, settings-failure-guidance]
affects: [apps/client/web/app/src/routes/(app)/agent/+page.svelte, apps/client/web/app/tests/component/agent-phase9.test.ts, apps/client/web/app/tests/e2e/agent.test.ts]
tech-stack:
  added: [vitest, playwright]
  patterns: [Svelte 5 runes, browser-route mocking, persisted-store fallback, keyed each blocks]
key-files:
  created: [apps/client/web/app/tests/component/agent-phase9.test.ts]
  modified: [apps/client/web/app/src/routes/(app)/agent/+page.svelte, apps/client/web/app/tests/e2e/agent.test.ts]
decisions:
  - "Keep New Chat thread-local only: set activeConversation immediately, clear message/input state, and do not mutate saved agent settings."
  - "Surface settings.json read failures as actionable guidance while still returning defaults so chat can continue."
metrics:
  duration: "~11m"
  completed_date: "2026-04-06"
---

# Phase 09 Plan 03: Agent New Chat Reset Semantics with Settings Retention Summary

New Chat now creates a fresh conversation thread immediately without wiping persisted agent configuration, and the Agent page now shows actionable settings-read guidance when `settings.json` cannot be loaded.

## Completed Work

### Task 1 — Make New Chat reset thread state while preserving settings

- Updated `createConversation()` to set `activeConversation = conv.id` immediately.
- Cleared thread-local state with `messages = []` and `inputText = ''`.
- Added visible `settings.json` read guidance while preserving default return values.
- Added component regression coverage for:
  - fresh-thread selection and empty-message reset
  - persisted `api_key` / `base_url` / `model` retention
  - fallback behavior when settings cannot be read

Commit: `0640c55`

### Task 2 — Add browser E2E coverage for fresh-thread and config retention semantics

- Added a Playwright case that seeds conversation state, clicks New Chat, and verifies the active thread switches to the new conversation while the prior message pane is cleared.
- Verified the settings form values remain stable across Agent ↔ Settings navigation.
- Kept the test aligned with the existing auth-guard-tolerant agent E2E style.

Commit: `37b600b`

## Verification

- `bun run --cwd apps/client/web/app test:unit -- tests/component/agent-phase9.test.ts` ✅
- `bun run --cwd apps/client/web/app test:e2e --project=desktop-chrome --grep "New Chat clears current thread and keeps settings state stable"` ✅
- `bun run --cwd apps/client/web/app test:e2e --grep "agent"` ⚠️ partial failure due missing Playwright Firefox/WebKit binaries in this environment

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Preserve settings-read guidance after conversation reloads**
- **Found during:** Task 1 validation
- **Issue:** The initial guidance message could be cleared by a subsequent successful `loadConversations()` call.
- **Fix:** Kept the settings-read guidance visible until replaced by a later error, so the user still gets actionable feedback.
- **Files modified:** `apps/client/web/app/src/routes/(app)/agent/+page.svelte`
- **Commit:** `0640c55`

## Deferred Issues

- Full `test:e2e --grep "agent"` could not complete in this environment because Firefox/WebKit Playwright executables were not installed. The new Agent E2E case passed on Chromium/desktop-chrome.

## Self-Check: PASSED

- FOUND: `.planning/phases/09-functional-correctness-baseline-fix/09-03-SUMMARY.md`
- FOUND: commit `0640c55`
- FOUND: commit `37b600b`
