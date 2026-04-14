# Auth Service

Auth domain service for OAuth login, session lifecycle, and token handling.

## Structure

- `src/domain/`: auth entities and domain errors
- `src/application/`: auth use cases and orchestration
- `src/policies/`: auth policy placeholders for timeout/retry/idempotency rules
- `src/ports/`: repository and provider abstractions
- `src/events/`: auth-related domain event placeholders
- `src/contracts/`: DTO re-exports and contract glue
- `migrations/`: schema migration placeholders and SQL files

## Verification

```bash
cargo test -p auth-service
cargo build -p auth-service
```
