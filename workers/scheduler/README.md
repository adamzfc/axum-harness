# Scheduler Worker

Time-based job dispatch using cron expressions. Evaluates schedules and dispatches registered jobs.

## Architecture

```
┌─────────────┐     ┌──────────────┐
│ Job Registry│────▶│ Dispatcher   │
│ (jobs/)     │     │ (dispatch/)  │
└─────────────┘     └──────────────┘
```

## Pattern

1. **Register** jobs with cron expressions
2. **Evaluate** schedules every minute
3. **Dispatch** due jobs to executors

## Health Checks

- `GET /healthz` — Liveness + dispatch count
- `GET /readyz` — Readiness probe

## Current State

- ✅ Job registry with cron support
- ✅ Job dispatcher with executor trait
- ⬜ Real cron evaluation (stub — logs only for now)
- ⬜ Real job implementations
