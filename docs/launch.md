# axum-harness: Agent-native backend architecture for Axum

**axum-harness** is a reference architecture and template for building production-grade Rust backends with Axum. It demonstrates how to structure a distributed backend so that AI agents can continuously take over, verify, and evolve it — without architectural drift.

## The problem

Traditional backend advice says: *"don't over-engineer, refactor when needed."* But in 2026, with AI agents as persistent co-developers, the cost model has changed. Agents don't struggle with complexity — they struggle with **implicit, unstable architecture**. Tribal knowledge, naming drift, inconsistent patterns, and rules that live in human memory instead of validators — these are what kill agent productivity.

## The approach: semantic-first, topology-late, agent-native

- **semantic-first** — Service-local semantics, contracts, CAS + idempotency, unified outbox, projection, and replay are designed upfront with explicit boundaries. Business logic is written once.
- **topology-late** — Deployment shape (single binary → multi-process → K3s cluster) is declared in a platform model YAML and can be switched without touching service code.
- **agent-native** — The repository ships with a built-in multi-agent collaboration protocol: 8 specialized agents, routing rules, scoped gates, and boundary checks.

The result: most future changes become **topology switches** and **adapter migrations**, not semantic rewrites.

## What's in the box

| Layer | What | Technology |
|-------|------|-----------|
| Web BFF | Sync request entrypoint | Axum, tokio |
| Services | DDD business logic (pure libraries) | domain → application → ports → infrastructure |
| Workers | Async executors: outbox relay, projector, indexer, scheduler | tokio, NATS |
| Platform model | YAML truth source for deployables, topologies, workflows, resources | Custom validators + generators |
| Contracts | API/Event/DTO/ErrorCode source of truth | Drift detection gates |
| Secrets | SOPS + Kustomize + Flux GitOps | No .env files |
| Agent harness | 8 specialized subagents with routing + gates | AGENTS.md, codemap.yml, gate matrix |

`counter-service` is the reference anchor — the smallest business unit with the most complete engineering chain:

```
service → contracts → server → CAS mutation + outbox (atomic)
  → outbox-relay → NATS → projector → read model → replay
```

## Quick start

```bash
just setup
just setup-deps
bash infra/local/scripts/bootstrap.sh up
just dev-api
just verify
```

## License

Apache 2.0

## Links

- **Repository**: [github.com/openclosed-org/axum-harness](https://github.com/openclosed-org/axum-harness)
- **Architecture docs**: [README](https://github.com/openclosed-org/axum-harness#readme)
- **Contributing**: [CONTRIBUTING.md](https://github.com/openclosed-org/axum-harness/blob/main/CONTRIBUTING.md)
