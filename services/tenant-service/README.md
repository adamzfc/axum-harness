# Tenant Service

> Reference module for multi-tenant ownership, multi-entity boundaries,
> onboarding workflow, member invitation, and compensation-aware mutation flows.

## Why This Module Exists

Use `tenant-service` as the reference when a feature includes:

1. tenant isolation
2. multiple entities under one aggregate boundary
3. workflow-driven state progression
4. compensation or long-running mutation semantics

## Key Files

1. `model.yaml` — service-local distributed semantics
2. `src/domain/` — tenant and membership rules
3. `src/application/` — commands and workflows entrypoints
4. `src/ports/` — persistence and external dependency traits
5. `platform/model/workflows/tenant-onboarding.yaml` — reference durable workflow

## Validation

```bash
cargo build -p tenant-service
cargo test -p tenant-service
```
