# packages/contracts/ — Structure Note

> **Date**: 2026-04-12
> **Status**: Current structure functional, target structure deferred to Phase 3-4

---

## Current Structure

```
packages/contracts/
├── api/          (contracts_api)     — HTTP API contracts
├── auth/         (contracts_auth)    — Authentication contracts  
├── events/       (contracts_events)  — Event schema contracts
├── errors/       (contracts_errors)  — Error code contracts
├── codegen/      — Code generation utilities
├── generated/    — Generated output (read-only)
├── protocols/    — Protocol-specific contracts
└── ui/           — UI-related contracts
```

## ARCHITECTURE.md Target Structure

```
packages/contracts/
├── http/         — HTTP request/response contracts
├── events/       — Event schema contracts
├── rpc/          — RPC/gRPC contracts
├── jsonschema/   — JSON Schema definitions
├── error-codes/  — Unified error codes
├── compat/       — Backward compatibility tests
└── sdk-gen/      — SDK generation configs
```

## Mapping Plan (For Future Implementation)

| Current | Target | Notes |
|---------|--------|-------|
| `api/` | `http/` | Rename, move HTTP types |
| `auth/` | SPLIT: `http/auth/` + `rpc/auth/` | Split OAuth types by protocol |
| `events/` | `events/` | Stays same |
| `errors/` | `error-codes/` | Rename |
| `codegen/` | `sdk-gen/` | Rename/restructure |
| `protocols/` | SPLIT across `http/`, `rpc/` | Distribute by protocol type |
| `generated/` | Keep | Generated output directory |
| `ui/` | Move to `packages/ui/` or delete | UI contracts don't belong here |

## Why Deferred

1. **High Risk**: Contracts are referenced by 10+ crates across services, servers, adapters
2. **Active Code**: These packages contain actual contract definitions, not stubs
3. **Better Timing**: Phase 3-4 is when runtime/authn/authz packages are created - contracts should be restructured alongside those implementations
4. **Current Structure Works**: No build issues, clear organization, easy to navigate

## Next Steps

When ready to restructure (Phase 3-4):
1. Create new target directories with stub Cargo.toml files
2. Move code incrementally (one sub-package at a time)
3. Run `cargo check --workspace` after each move
4. Update all references in services, servers, adapters
5. Verify contract generation still works

---

**Phase 2 Decision**: Keep current structure, document target for future work.
