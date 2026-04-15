# Web BFF

> Reference synchronous entrypoint for user-facing surfaces.
> Aggregates the `counter`, `tenant`, and `settings` reference services into view-oriented APIs.

## Scope

1. HTTP adaptation only
2. auth / tenant / trace context injection
3. response composition
4. no domain logic
5. no long transaction orchestration

## Reference Coverage

1. user-facing synchronous query path
2. command endpoint delegating to service layer
3. consistency-aware read path
