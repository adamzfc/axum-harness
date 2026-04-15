# Migration runner

Phase 1: CLI tool to run SQL migrations against Turso/LibSQL databases. Migrations live per-service under `services/<name>/migrations/`.

```
just migrate up        # Apply all pending migrations
just migrate down N    # Rollback N migrations
just migrate status    # Show migration status
```
