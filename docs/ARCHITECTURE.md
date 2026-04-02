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

### crates

Rust-only reuse layer. Only enabled when sufficient Rust-only code exists.

```
crates/
  rust-only/          # Placeholder for Rust-only crates
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

## Boundary Rules

- `core` must not depend on `apps`
- `features` must not depend on specific host apps
- `adapters` must not carry business logic
- `contracts` must not be polluted by implementation details
- `workers` must not bypass contracts to define custom event schemas
- `apps/client/web/hosts/*` can only do host adaptation, not copy business logic

## Why This Structure

- Keeps core logic independent from delivery mechanism (desktop/server/extension).
- Allows incremental backend/runtime adoption without rewiring everything.
- Supports small-team velocity by making module responsibilities explicit.
- Enables agent-friendly development with clear boundaries and patterns.

## Current State

The repository is at scaffold level:

- Layer boundaries exist.
- Packages and wiring points are prepared.
- Most business/runtime implementation is left for adopters.

## Suggested Growth Path

1. Define domain entities and invariants in `packages/core/domain`.
2. Add use case services in `packages/core/usecases`.
3. Define request/response contracts in `packages/contracts`.
4. Implement features in `packages/features/<feature-name>`.
5. Connect adapters in `packages/adapters/hosts/tauri` and `servers/api`.
6. Expose via Tauri commands or HTTP endpoints.

## Quality Gates

The default CI enforces:

- Rust `check`, `fmt`, `clippy`, `test`
- Frontend `check`, `lint`, `build`

See `.github/workflows/ci.yml` for exact commands.
