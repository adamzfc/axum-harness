---
phase: 07-frontend-type-consumption
plan: 02
subsystem: auth
tags: [ts-rs, typegen, svelte-5, typescript, contracts]

# Dependency graph
requires:
  - phase: 07-01
    provides: "Typegen infrastructure and initial generated types (TokenPair, UserSession, OAuthCallback)"
provides:
  - "UserProfile type added to contracts/auth and generated to frontend"
  - "ipc/auth.ts uses generated types instead of inline definitions"
  - "auth.svelte.ts imports UserProfile from generated, eliminating duplicate type chain"
affects: [auth, frontend, typegen, contracts]

# Tech tracking
tech-stack:
  added: [UserProfile struct in contracts_auth]
  patterns: ["Frontend AuthSession composes generated TokenPair + UserProfile rather than duplicating fields"]

key-files:
  created:
    - apps/client/web/app/src/lib/generated/auth/UserProfile.ts
    - packages/contracts/auth/bindings/auth/UserProfile.ts
  modified:
    - packages/contracts/auth/src/lib.rs
    - apps/client/web/app/src/lib/ipc/auth.ts
    - apps/client/web/app/src/lib/stores/auth.svelte.ts

key-decisions:
  - "AuthSession kept as frontend composition type combining TokenPair + UserProfile (not in contracts)"
  - "UserProfile.sub vs UserSession.user_sub naming difference is intentional — different semantic contexts"

patterns-established:
  - "Frontend types import from $lib/generated/ for contract-aligned types"
  - "Local composition types (AuthSession) combine multiple generated types for client-side convenience"

requirements-completed: [CONTRACT-02]

# Metrics
duration: 3min
completed: 2026-04-03
---

# Phase 07 Plan 02: Frontend Type Consumption Summary

**Replace inline auth types with ts-rs generated types, closing contracts → frontend type consumption loop**

## Performance

- **Duration:** ~3 min
- **Started:** 2026-04-03T00:18:00Z
- **Completed:** 2026-04-03T00:21:16Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments

- UserProfile struct added to contracts/auth/src/lib.rs with ts-rs export
- TypeScript type generated at frontend/generated/auth/UserProfile.ts
- ipc/auth.ts inline UserProfile/AuthSession replaced with generated imports
- auth.svelte.ts updated to use generated UserProfile and new AuthSession structure
- All session field accesses updated (expires_at → tokens.expires_in)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add UserProfile to contracts and generate** - `a320735` (feat)
2. **Task 2: Replace inline types with generated in ipc/auth.ts** - `379a26c` (feat)
3. **Task 3: Update auth.svelte.ts to use generated types** - `f532194` (feat)

**Plan metadata:** docs commit pending (final metadata commit)

## Files Created/Modified

- `packages/contracts/auth/src/lib.rs` - Added UserProfile struct + export test
- `apps/client/web/app/src/lib/generated/auth/UserProfile.ts` - Generated TS type
- `packages/contracts/auth/bindings/auth/UserProfile.ts` - Generated binding
- `apps/client/web/app/src/lib/ipc/auth.ts` - Replaced inline types with generated imports, restructured AuthSession
- `apps/client/web/app/src/lib/stores/auth.svelte.ts` - Updated imports and field access patterns

## Decisions Made

- **AuthSession as composition type:** Kept AuthSession as a frontend-local interface that composes `TokenPair` + `UserProfile` from generated types, rather than trying to replicate the full structure in contracts. This avoids duplication while keeping the frontend's convenience type.
- **UserProfile.sub vs UserSession.user_sub:** These are intentionally different — UserProfile is the full user info from OAuth provider, UserSession is a minimal session identifier. Both coexist, used in different contexts.

## Deviations from Plan

**1. [Rule 1 - Bug] Fixed session field access after AuthSession restructuring**
- **Found during:** Task 3 (auth.svelte.ts update)
- **Issue:** After restructuring AuthSession to use `tokens: TokenPair`, the old code still accessed `session.expires_at` and `session.access_token` directly
- **Fix:** Updated `checkSession()` and `setSession()` to use `session.tokens.expires_in`
- **Files modified:** `apps/client/web/app/src/lib/stores/auth.svelte.ts`
- **Verification:** svelte-check passes for auth files (pre-existing ToolCall.ts error unrelated)
- **Committed in:** `f532194` (Task 3 commit)

---

**Total deviations:** 1 auto-fixed (1 bug fix for field access after restructuring)
**Impact on plan:** Essential correctness fix — old field names no longer exist on the restructured type.

## Issues Encountered

- Pre-existing svelte-check error in `generated/api/ToolCall.ts` (missing `serde_json/JsonValue` module) — unrelated to auth changes, not fixed (out of scope per deviation rules)
- `npx tsc` not available directly — used `svelte-check` via node_modules instead

## Next Phase Readiness

- Contracts → frontend type consumption chain is now closed for auth types
- Ready for 07-03 (remaining type consumption tasks)
- UserProfile now available for any component that needs user display info

---

*Phase: 07-frontend-type-consumption*
*Completed: 2026-04-03*
