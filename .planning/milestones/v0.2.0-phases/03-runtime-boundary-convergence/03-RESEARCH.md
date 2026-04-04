# Phase 03 Research: Runtime 边界收敛

**Researched:** 2026-04-02
**Confidence:** HIGH
**Phase:** 03-runtime-boundary-convergence
**Requirements:** RUNTIME-01, RUNTIME-02, RUNTIME-03

## Current Architecture (Migration Source)

### Dependency Graph (Current — Non-Compliant)

```
domain ← usecases ← servers/api (runtime_server)
domain ← usecases ← apps/client/native/src-tauri
contracts_api ← usecases           ← ❌ VIOLATION
contracts_api ← servers/api        ← ✓ OK
port impls in servers/api          ← ❌ VIOLATION (TenantAwareSurrealDb, TursoDb)
port impls in native-tauri         ← ❌ VIOLATION (EmbeddedLibSql)
```

### Port Implementations to Migrate

| Implementation | Current Location | Target | Trait |
|---------------|-----------------|--------|-------|
| `TenantAwareSurrealDb` | `servers/api/src/ports/surreal_db.rs` | `packages/adapters/storage/surrealdb` | `SurrealDbPort` |
| `run_tenant_migrations` (SurrealDB) | `servers/api/src/ports/surreal_db.rs` | `packages/adapters/storage/surrealdb` | N/A (standalone) |
| `TursoDb` | `servers/api/src/ports/turso_db.rs` | `packages/adapters/storage/libsql` | `LibSqlPort` |
| `run_tenant_migrations` (Turso) | `servers/api/src/ports/turso_db.rs` | `packages/adapters/storage/libsql` | N/A (standalone) |
| `EmbeddedLibSql` | `apps/client/native/src-tauri/src/ports/lib_sql.rs` | `packages/adapters/storage/libsql` | `LibSqlPort` |
| `run_tenant_migrations` (Embedded) | `apps/client/native/src-tauri/src/ports/lib_sql.rs` | `packages/adapters/storage/libsql` | N/A (standalone) |

### usecases Current Dependencies

```
usecases/Cargo.toml:
  domain = { workspace = true }
  contracts_api = { workspace = true }  ← ❌ Must remove
  async-trait, serde, serde_json, chrono, thiserror
```

**tenant_service.rs** uses: `domain::ports::lib_sql::LibSqlPort`, `domain::ports::surreal_db::SurrealDbPort` (OK) — does NOT import from contracts_api directly, but the Cargo.toml dependency is the violation.

### native-tauri Commands to Migrate to runtime_tauri

| Command | File | Business Logic Level |
|---------|------|---------------------|
| `start_oauth` | `commands/auth.rs` | Host-specific (uses tauri-plugin-opener) |
| `handle_oauth_callback` | `commands/auth.rs` | Host-specific (uses tauri-plugin-store) |
| `get_session` | `commands/auth.rs` | Host-specific (uses tauri-plugin-store) |
| `quit_app` | `commands/auth.rs` | Host-specific |
| `get_config` | `commands/config.rs` | Host-specific (uses dotenvy) |
| `sync_start/stop/once/get_stats/configure` | `commands/sync.rs` | Business logic (calls SyncEngine) |
| `start_refresh_timer` | `commands/auth.rs` | Business logic (token refresh) |

### SyncEngine (Business Logic in Host)

```
apps/client/native/src-tauri/src/sync/
  mod.rs      — SyncConfig, ConflictStrategy
  engine.rs   — SyncEngine (core sync logic)
  conflict.rs — Conflict resolution
```

This is business logic living in the host app. In a strict hexagonal architecture, SyncEngine would be in core. But for Phase 3, we focus on command bridge migration — SyncEngine can stay in native-tauri or be moved in a later phase.

## Target Architecture

### Target Dependency Graph

```
domain ← usecases (usecases NO LONGER depends on contracts_api)
domain ← adapters/storage/surrealdb
domain ← adapters/storage/libsql
domain ← runtime_tauri ← native-tauri
usecases ← adapters/storage/*
usecases ← runtime_tauri
usecases ← servers/api
contracts_api ← servers/api (route handler maps DTOs)
contracts_api ← features/*
core + adapters + contracts ← features/*
```

### New Crates to Create

| Crate | Path | Dependencies | Contents |
|-------|------|-------------|----------|
| `storage_surrealdb` | `packages/adapters/storage/surrealdb/` | domain, surrealdb, async-trait, serde, serde_json | TenantAwareSurrealDb + migrations |
| `storage_libsql` | `packages/adapters/storage/libsql/` | domain, libsql, async-trait, serde, serde_json, base64 | TursoDb + EmbeddedLibSql + migrations |
| `runtime_tauri` | `packages/adapters/hosts/tauri/` (exists, needs content) | domain, usecases, tauri | Tauri command handlers as bridge to usecases |

### Feature Crates (Skeleton Only — Phase 3)

| Crate | Path | Dependencies | Contents |
|-------|------|-------------|----------|
| `feature_auth` | `packages/features/auth/` | domain, usecases, contracts_api, runtime_tauri | Skeleton lib.rs |
| `feature_admin` | `packages/features/admin/` | domain, usecases, contracts_api | Skeleton lib.rs |
| `feature_counter` | `packages/features/counter/` | domain, usecases | Skeleton lib.rs |
| `feature_agent` | `packages/features/agent/` | domain, usecases | Skeleton lib.rs |

## Migration Approach

### Key Decisions from Research

1. **Port implementations move verbatim** — trait impl signatures stay identical, only crate path changes
2. **usecases internal types** — usecases already defines its own `Tenant`, `CreateTenantInput`, etc. Remove the `contracts_api` dependency from Cargo.toml
3. **Route handler mapping** — `servers/api/src/routes/tenant.rs` keeps `use contracts_api::InitTenantRequest` (that's the server boundary), and calls usecases with its own types
4. **runtime_tauri command bridge** — Tauri command handlers call usecases, not business logic directly
5. **Feature crates are skeletons** — Cargo.toml + lib.rs only, actual implementation in Phase 4

### Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| LibSqlPort impls (TursoDb vs EmbeddedLibSql) in same crate | Compilation conflicts (different libsql features) | Same crate, feature-gated: `embedded` vs `remote` features |
| SyncEngine stays in native-tauri | Temporary boundary violation | Acceptable for Phase 3, move in Phase 4+ |
| Many workspace members added | Slower workspace resolution | Expected, standard for hexagonal architecture |
| cargo-deny configuration | May block valid deps | Start with minimal rules, expand iteratively |

## Validation Architecture

### Boundary Check Strategy

1. **cargo-deny deny.toml** — Hard dependency direction rules:
   - `domain` must NOT depend on: adapters, hosts, contracts_api, servers
   - `usecases` must NOT depend on: adapters, hosts, contracts_api, servers
   - `contracts_api` must NOT depend on: domain, usecases, adapters, hosts

2. **CI script** — `cargo tree` based verification:
   - Parse `cargo tree -p domain --depth 1` — no adapter/host crates
   - Parse `cargo tree -p usecases --depth 1` — no contracts_api
   - Parse `cargo tree -p contracts_api --depth 1` — no domain/usecases

3. **Agent rubric** — `.agents/rubrics/boundary-compliance.md` semantic spec

### Compilation Gate

All new crates must compile: `cargo check --workspace` passes after each plan.

## Sources

- `servers/api/src/ports/surreal_db.rs` — TenantAwareSurrealDb (360 lines, 14 tests)
- `servers/api/src/ports/turso_db.rs` — TursoDb (129 lines, 1 test)
- `apps/client/native/src-tauri/src/ports/lib_sql.rs` — EmbeddedLibSql (256 lines, 7 tests)
- `packages/core/usecases/src/tenant_service.rs` — TenantService (318 lines, 4 tests)
- `apps/client/native/src-tauri/src/lib.rs` — native-tauri entry point (193 lines)
- `apps/client/native/src-tauri/src/commands/` — auth.rs (422 lines), config.rs (49 lines), sync.rs (154 lines)
- `packages/adapters/hosts/tauri/src/lib.rs` — runtime_tauri placeholder (1 line)
- `Cargo.toml` — workspace config
- `.planning/phases/03-runtime-boundary-convergence/03-CONTEXT.md` — 20 decisions

---

*Research completed: 2026-04-02*
*Ready for planning: yes*
