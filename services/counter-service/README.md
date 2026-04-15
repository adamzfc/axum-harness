# Counter Service

> Reference module for the smallest complete business chain.
> Covers: single aggregate ownership, CAS-style versioning, command handling,
> event publication, projection/replay-friendly design, and service-local semantics.

## Why This Module Exists

Use `counter-service` as the first copy target when introducing a new bounded service.
It is intentionally small and should remain the easiest end-to-end example in the repo.

## Key Files

1. `model.yaml` — source of truth for service-local distributed semantics
2. `src/domain/` — aggregate and domain rules
3. `src/application/` — use cases
4. `src/ports/` — external dependency abstractions
5. `src/events/` — event definitions local to the service

## Validation

```bash
cargo test -p counter-service
cargo build -p counter-service
```

## Reference Pattern Coverage

1. Single aggregate owner
2. CAS-style version field
3. Command with idempotency key
4. Tenant-scoped event publication
5. Projection/replay compatibility
