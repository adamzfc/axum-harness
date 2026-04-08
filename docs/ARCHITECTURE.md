# Architecture

This template follows a boundary-first architecture so teams can evolve features
without coupling UI, business rules, and runtime concerns too early.

## High-level Layers

### apps

Executable product shells. These do not carry core business logic—they compose features and handle host-specific startup.

```
apps/
  client/
    desktop/          # Tauri desktop app
    web/              # SvelteKit web app
      app/            # Canonical SvelteKit application
      hosts/          # Host-specific adapters (telegram-miniapp, farcaster-miniapp, etc.)
    browser-extension/# Browser extension shell
  ops/
    docs-site/        # Documentation site
    storybook/        # Component storybook
```

### servers

Network-facing services. Each server has a distinct role.

```
servers/
  api/                # Main API server (Axum)
  gateway/            # API gateway
  realtime/           # WebSocket/realtime server
```

### workers

Background jobs, protocol ingestion, chain indexing, async processing.

```
workers/
  protocols/          # Protocol workers (atproto, farcaster, nostr)
  chains/             # Chain indexers (evm, base, solana, ton)
  jobs/               # Background jobs (notifications, media, search, sync)
```

### packages/core

Pure business rules. Must not depend on host, protocol, or chain.

```
packages/core/
  domain/             # Entities, value objects, invariants, policies
  usecases/           # Use case orchestration, command/query handlers
  state/              # Session state, cache policy, sync markers
```

### packages/features

Business capability modules. Each feature combines core + contracts + adapters.

```
packages/features/
  auth/
  profile/
  feed/
  social-graph/
  wallet/
  payments/
  notifications/
  admin/
```

### packages/adapters

Translation layer for all external world integrations.

```
packages/adapters/
  hosts/              # Host adapters (tauri, browser-extension, miniapps)
  protocols/          # Protocol adapters (atproto, farcaster, nostr)
  chains/             # Chain adapters (evm, base, solana, ton)
  storage/            # Storage adapters (indexeddb, sqlite, libsql, tauri-store)
  auth/               # Auth adapters (oauth, passkey, dpop)
  telemetry/          # Telemetry adapters (tracing, otel)
```

### packages/contracts

Single source of truth for all system contracts.

```
packages/contracts/
  api/                # REST/HTTP/IPC request/response models
  auth/               # Session, proof, credential models
  events/             # Internal events, queue messages
  errors/             # Standard error shapes and codes
  protocols/          # External protocol schema wrappers
  ui/                 # UI-facing state/form/token schemas
  codegen/            # Code generation configuration
```

### packages/ui

Design system, component primitives, icons, tokens.

```
packages/ui/
  design-system/
  web/
  icons/
  tokens/
```

### packages/shared

Shared utilities not tied to business rules.

```
packages/shared/
  config/
  env/
  utils/
  testing/
  types/
```

### tools

Generators, MCP servers, eval datasets, repo scripts.

```
tools/
  scripts/
  generators/
  mcp/
    servers/
    clients/
  evals/
    datasets/
    graders/
    suites/
```

### .agents

Agent governance layer—not just documentation.

```
.agents/
  skills/
  prompts/
  playbooks/
  rubrics/
```

## Strict Dependency Rules

These rules are **enforced by CI** (`boundary-check.ts`, `contracts-check`). Violations block merge.

### Dependency Direction (MUST)

```
contracts/        ← Single Source of Truth for all types (DTOs, schemas, events)
     ↑
features/         ← Trait + type definitions at feature boundary (NO implementation)
     ↑
usecases/         ← Concrete service implementations (depends on features + domain)
     ↑
adapters/         ← External world translations (storage, auth, hosts, protocols)
     ↑
apps / servers    ← Composition layer (wires everything together)
```

**MUST rules:**

1. **`feature-*` crates MUST NOT depend on `usecases`**
   - Features define traits; usecases implement them
   - Reverse dependency breaks the inversion-of-control pattern
   - Exception: none

2. **`contracts/` MUST be the Single Source of Truth for shared types**
   - If the same concept exists in `features/` and `contracts/`, the feature-layer type MUST reference or derive from the contracts type
   - No field-level drift between duplicate types (e.g. `i64` vs `u64`)
   - DTOs in `contracts/` carry `ts_rs::TS` + `utoipa::ToSchema`; feature-internal types must not duplicate this

3. **`domain/` MUST contain only port traits and value objects**
   - No concrete implementations
   - No transport or storage dependencies

4. **`usecases/` MUST implement traits defined in `features/`**
   - All concrete business logic lives here
   - Depends on `domain` (ports), `feature-*` (trait definitions), `contracts/*` (DTOs)

5. **`adapters/` MUST NOT carry business logic**
   - Translation only: external protocol → internal contract
   - No invariants, no policies, no orchestration

6. **`apps/` / `servers/` MUST NOT contain business logic**
   - Composition, wiring, host-specific startup only
   - Delegate all work to `usecases/` via traits

### Dependency Violations (known, to be fixed)

| Violation | Location | Status |
|-----------|----------|--------|
| `feature-auth` depends on `usecases` | `packages/features/auth/Cargo.toml` | ⚠️ Pending fix |
| Unused deps in `feature-counter`, `feature-admin` | `packages/features/*/Cargo.toml` | ⚠️ Pending fix |
| `DashboardStats` type drift (`i64` vs `u64`) | `feature-admin` vs `contracts/api` | ⚠️ Pending fix |
| `UserProfile` duplicated in feature + contracts | `feature-auth` vs `contracts/auth` | ⚠️ Pending fix |

## Actual Dependency Graph (as of 2026-04-08)

```
contracts_api ──┐
contracts_auth ─┤
contracts_events┤                  (pure DTOs, ts_rs + utoipa)
                │
domain ─────────┤                  (port traits: LibSqlPort, SurrealDbPort)
                │
   ┌────────────┴────────────┐
   │                         │
feature-counter         feature-agent      feature-admin    feature-auth
(trait + types)         (trait + types)    (trait + types)  (trait + types)
   │                         │                   │              │
   └────────────┬────────────┘                   │              │
                │                                │              │
            usecases ────────────────────────────┘              │
   (LibSqlCounterService, LibSqlTenantService,                  │
    LibSqlAdminService, LibSqlAgentService)                     │
                │                                               │
                └───────────────────────────────────────────────┘
                                    ↑
                            adapters (storage, hosts, auth)
```

## Current State

The repository has moved beyond scaffold level:

- **4 features** have trait definitions (`counter`, `agent`, `admin`, `auth`)
- **4 usecase implementations** are complete with tests:
  - `counter_service` — LibSQL backend + 3 tests
  - `tenant_service` — LibSQL + SurrealDB backends + 4 tests
  - `admin_service` — Aggregation layer + tests
  - `agent_service` — OpenAI streaming SSE + tool execution + 3 tests
- **Contracts** generate TypeScript types via `ts-rs` with drift-check CI gate
- **Boundary enforcement** via `boundary-check.ts` in CI
- Known dependency violations are tracked above

## Why This Structure

- Keeps core logic independent from delivery mechanism (desktop/server/extension).
- Allows incremental backend/runtime adoption without rewiring everything.
- Supports small-team velocity by making module responsibilities explicit.
- Enables agent-friendly development with clear boundaries and patterns.
- CI-enforced boundaries prevent architectural drift over time.

## Suggested Growth Path

### For new features:

1. Define trait + error + types in `packages/features/<feature-name>/src/lib.rs`.
2. Add DTOs to `packages/contracts/<domain>/src/lib.rs` (with `ts_rs::TS` + `utoipa::ToSchema`).
3. Run `repo:typegen` to sync TypeScript types.
4. Implement the trait in `packages/core/usecases/src/<feature>_service.rs`.
5. Wire the adapter in `packages/adapters/<type>/`.
6. Expose via Tauri command or Axum handler.

### For new adapters:

1. Define port trait in `packages/core/domain/src/ports/` if not exists.
2. Implement in `packages/adapters/<type>/<adapter-name>/`.
3. Inject into usecase service via trait bound.
4. Add tests with mock port implementation.

## Quality Gates

The default CI enforces:

- Rust `check`, `fmt`, `clippy`, `test`
- Frontend `check`, `lint`, `build`

See `.github/workflows/ci.yml` for exact commands.
