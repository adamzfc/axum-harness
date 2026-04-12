# Sync-Reconciler Worker

Sync conflict resolution and data reconciliation between sources.

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│ Plans       │────▶│ Executors    │────▶│ Conflict    │
│ (plans/)    │     │ (executors/) │     │ (conflict/) │
└─────────────┘     └──────────────┘     └─────────────┘
```

## Pattern

1. **Define** reconciliation plans (source, target, strategy)
2. **Execute** plans periodically
3. **Resolve** conflicts based on strategy (SourceWins, TargetWins, LastWriteWins, Manual)

## Health Checks

- `GET /healthz` — Liveness + reconcile count
- `GET /readyz` — Readiness probe

## Current State

- ✅ Reconciliation plan framework
- ✅ Conflict resolver with strategy support
- ✅ Stub executor
- ⬜ Real data source comparison
- ⬜ Real reconciliation execution
