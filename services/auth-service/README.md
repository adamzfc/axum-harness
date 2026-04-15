# Auth Service

> Planned auth capability placeholder.

## Status

1. This crate is a stub, not a reference module.
2. `model.yaml` reserves auth-session ownership and login/session semantics.
3. Existing source code is legacy scaffold and should not be copied into new services.

## Current Scope

1. OAuth login/session lifecycle experiments
2. Session/token persistence adapters
3. Future auth capability reservation for the harness

## Verification

```bash
cargo test -p auth-service
cargo build -p auth-service
```
