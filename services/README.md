# Services

> `services/` contains business capability and state-boundary libraries.
> In the current harness, service directories fall into three classes:
> `reference`, `stub`, and `deprecated`.

## Current Inventory

| Directory | Class | Purpose |
|---|---|---|
| `counter-service/` | reference | Smallest complete command/query/event sample |
| `tenant-service/` | reference | Multi-entity, workflow-driven tenant sample |
| `auth-service/` | stub | Planned auth capability placeholder |
| `user-service/` | stub | Planned identity/profile capability placeholder |
| `indexing-service/` | stub | Planned indexing/search capability placeholder |
| `event-bus/` | deprecated | Transitional infrastructure facade pending package/worker migration |

## Rules

1. Every service directory must carry a `model.yaml` that explains its current semantic status.
2. Only `counter-service` and `tenant-service` are copy targets for new business services.
3. Stub services may keep minimal or legacy code, but they are not reference modules and should not drive new design decisions.
4. `event-bus/` is transitional only; new messaging behavior belongs in shared packages and workers.

## Reference Skeleton

Reference services should converge on this shape:

```text
services/<name>/
├── model.yaml
├── Cargo.toml
├── src/
│   ├── domain/
│   ├── application/
│   ├── policies/
│   ├── ports/
│   ├── events/
│   ├── contracts/
│   └── lib.rs
├── tests/
├── migrations/
└── README.md
```

Concrete adapter modules can remain temporarily while existing servers/apps/workers still depend on them, but they are legacy outer-edge code, not the target template.
