# {{domain}} Service

> {{description}}

## Status

1. Decide first whether this directory is a `reference`, `stub`, or `planned` service.
2. New business capabilities should usually start by copying `counter-service` or `tenant-service`, not by improvising structure.

## Required Files

1. `model.yaml` — service-local distributed semantics
2. `src/domain/` — entities, value objects, invariants
3. `src/application/` — command/query orchestration
4. `src/ports/` — external dependency abstractions
5. `src/events/` — service-local event intent
6. `src/policies/` — service-local policy hooks
7. `src/contracts/` — shared contract glue
8. `tests/` and `migrations/`

## Required Questions

1. Which entities does this service own?
2. Which commands require idempotency keys?
3. Which events are replayable?
4. Which queries need strong or read-your-write consistency?
5. Which cross-service reads are allowed and why?

## Verification

```bash
cargo build -p {{domain}}-service
cargo test -p {{domain}}-service
just validate-state strict
```
