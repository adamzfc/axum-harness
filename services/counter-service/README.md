# Counter Service — Golden Module 🏆

> **This is the reference implementation.** New services should copy this directory structure and conventions.

## Status

- [x] Phase 0: Domain + application + ports implemented
- [x] Phase 0: LibSqlCounterRepository adapter implemented
- [x] Phase 0: Unit + integration tests passing
- [ ] Phase 1: Independent deployment (HTTP/gRPC interface)
- [ ] Phase 2: NATS event publishing on counter changes

## Quick Start

```bash
cargo test -p counter-service          # all tests
cargo test -p counter-service -- unit  # unit tests only
cargo test -p counter-service -- integration # integration tests only
cargo build -p counter-service         # compile check
```

## Architecture

```
counter-service/
├── src/
│   ├── domain/        # Counter entity, CounterId, errors (zero deps)
│   ├── application/   # TenantScopedCounterService (orchestrates via ports)
│   ├── ports/         # CounterRepository trait (storage abstraction)
│   ├── contracts/     # DTO re-exports from packages/contracts/
│   ├── sync/          # OfflineFirst sync strategies (stub)
│   ├── infrastructure/# LibSqlCounterRepository (concrete impl)
│   ├── interfaces/    # HTTP/gRPC stubs (BFF handles HTTP)
│   └── lib.rs         # Public barrel
├── tests/
│   ├── unit/          # Mock-based tests (fast, deterministic)
│   └── integration/   # Real libsql tests (full stack)
├── migrations/        # SQL DDL (idempotent)
└── Cargo.toml
```

## Dependency Rules

| Layer | May depend on | Must NOT depend on |
|-------|--------------|-------------------|
| `domain/` | `serde`, `chrono`, `thiserror` | async, storage, HTTP |
| `ports/` | `domain/`, `async-trait` | concrete storage |
| `application/` | `domain/`, `ports/`, feature traits | concrete storage, HTTP |
| `infrastructure/` | `ports/`, `domain/`, storage crates | feature traits, HTTP |
| `contracts/` | `packages/contracts/` | any internal module |

## Creating a New Service

1. Copy this directory: `cp -r services/counter-service services/my-service`
2. Replace `counter` with `my-service` in all files
3. Update `Cargo.toml` package name and feature trait dependency
4. Define your domain entities in `domain/`
5. Define your repository port in `ports/`
6. Implement use cases in `application/`
7. Add concrete adapter in `infrastructure/`
8. Write tests in `tests/unit/` and `tests/integration/`
