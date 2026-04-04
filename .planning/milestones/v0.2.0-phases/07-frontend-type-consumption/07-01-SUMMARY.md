---
phase: 07-frontend-type-consumption
plan: 01
subsystem: typegen
tags: [ts-rs, typegen, moon, contracts, typescript]

# Dependency graph
requires:
  - phase: 02-contracts-typegen
    provides: contracts crate structure with ts-rs exports, typegen pipeline
provides:
  - All 11 contract types generated to frontend generated/ directory
  - bigint type issue fixed in TokenPair expires_in
  - Drift check passing with no sync issues
affects: [frontend-type-consumption plans 02-03, feature development needing typed contracts]

# Tech tracking
tech-stack:
  added: []
  patterns: [ts-rs #[ts(type = "number")] for i64→number override]

key-files:
  created:
    - apps/client/web/app/src/lib/generated/api/ChatMessage.ts
    - apps/client/web/app/src/lib/generated/api/ToolCall.ts
    - apps/client/web/app/src/lib/generated/api/AgentConfig.ts
  modified:
    - packages/contracts/auth/src/lib.rs
    - apps/client/web/app/src/lib/generated/auth/TokenPair.ts
    - packages/contracts/auth/bindings/auth/TokenPair.ts

key-decisions:
  - "Use #[ts(type = \"number\")] for i64 expires_in to avoid bigint JSON serialization issues"

patterns-established:
  - "bigint override pattern: #[ts(type = \"number\")] for i64 fields that need JSON serialization"

requirements-completed: [CONTRACT-02, CONTRACT-01]

# Metrics
duration: 3min
completed: 2026-04-03
---

# Phase 07 Plan 01: Fix Typegen Pipeline Summary

**修复 typegen 管道，11 个 contracts 类型完整生成到前端，修复 bigint 兼容性问题**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-03T00:15:00Z
- **Completed:** 2026-04-03T00:18:00Z
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments
- 运行 `moon run repo:typegen` 成功生成所有 contracts 类型
- 验证 11 个类型文件全部存在于 frontend generated/ 目录
- 修复 TokenPair.expires_in 的 bigint 问题（改为 number）
- repo:contracts-check 通过，无 drift

## Task Commits

Each task was committed atomically:

1. **Task 1: 运行 typegen 并验证所有类型已生成** - `284dcfb` (feat)
2. **Task 2: 修复 ts-rs bigint 类型问题** - `9dd2e21` (fix)
3. **Task 3: 运行 drift check 验证类型同步一致性** - `06348b6` (chore)

## Files Created/Modified
- `apps/client/web/app/src/lib/generated/api/ChatMessage.ts` - ChatMessage 类型定义
- `apps/client/web/app/src/lib/generated/api/ToolCall.ts` - ToolCall 类型定义
- `apps/client/web/app/src/lib/generated/api/AgentConfig.ts` - AgentConfig 类型定义
- `packages/contracts/auth/src/lib.rs` - 添加 #[ts(type = "number")] 到 expires_in
- `apps/client/web/app/src/lib/generated/auth/TokenPair.ts` - 重新生成为 number 类型
- `packages/contracts/auth/bindings/auth/TokenPair.ts` - 同步更新为 number 类型

## Decisions Made
- 使用 `#[ts(type = "number")]` 注解覆盖 i64→bigint 的默认映射，因为 bigint 不能直接用于 JSON 序列化

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] 修复 TokenPair expires_in 的 bigint 类型**
- **Found during:** Task 1 (typegen 验证)
- **Issue:** Rust i64 默认映射为 TypeScript bigint，但 bigint 不能用于 JSON 序列化
- **Fix:** 在 contracts/auth/src/lib.rs 中添加 `#[ts(type = "number")]` 注解
- **Files modified:** packages/contracts/auth/src/lib.rs, TokenPair.ts (两处)
- **Verification:** `grep -r "bigint" apps/client/web/app/src/lib/generated/` 返回空
- **Committed in:** 9dd2e21 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 missing critical)
**Impact on plan:** 修复是类型可用性的必要条件，无范围蔓延。

## Issues Encountered
- `packages/contracts/generated/` 目录被 .gitignore 忽略，但 `packages/contracts/auth/bindings/` 被 git 跟踪，需要同步更新

## Next Phase Readiness
- 所有 contracts 类型已生成到前端，可以进行类型消费
- 后续 plan 可以直接导入这些类型使用

---
*Phase: 07-frontend-type-consumption*
*Completed: 2026-04-03*
