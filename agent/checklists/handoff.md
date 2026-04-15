# Handoff Checklist

Use this checklist before handing work from one subagent lane to another.

## Service → Server

1. `services/<name>/model.yaml` reflects the new commands, events, and queries.
2. Service README explains whether the change affects reference semantics or only implementation.
3. Contracts that the server depends on are updated first.

## Service → Worker

1. Published events are declared replayable or explicitly non-replayable.
2. Projection/rebuild implications are documented in README or verification files.
3. Idempotency and ordering scope are present before worker consumption logic changes.

## Platform → Service/Worker

1. Global owner, consistency, or idempotency defaults are updated in `platform/model/state/*`.
2. Workflow changes include checkpoint and compensation semantics.
3. Deployable/topology changes do not silently change state semantics.

## Final Verification

1. Run `bun run scripts/verify-handoff.ts <subagent>`.
2. Run `just verify` before convergence when the change spans multiple lanes.
