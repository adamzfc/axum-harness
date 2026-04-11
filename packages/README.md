# Packages

Shared layer — cross-platform reusable components. This is the heart of the architecture.

## Layer Rules

1. **No composition logic** — packages define and implement contracts, they don't wire things together
2. **Dependency direction is strict** — see the table below
3. **Everything here is independently testable**

## Structure

```
packages/
├── adapters/     # External protocol translators (DB, OAuth, Tauri, telemetry)
├── contracts/    # Single source of truth for shared types (DTOs, events, errors)
├── core/         # Pure business logic (domain ports + usecase implementations)
├── features/     # Trait definitions per feature (what the system can do)
├── shared/       # Technical utilities (errors, utils, config, tracing)
├── ui/           # Frontend component kit (Svelte components)
└── api-contracts/# SDK generation config (ts-rs, prost, utoipa)
```

## Dependency Direction

```
contracts/  ←  Types only
    ↑
features/   ←  Traits + types, NO implementations
    ↑
usecases/   ←  Implements feature traits
    ↑
adapters/   ←  Protocol translation
    ↑
servers/ / apps/  ←  Composition
```

Violating this direction is a **build failure** (enforced by `just quality boundary`).
