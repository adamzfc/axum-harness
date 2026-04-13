# Topology ↔ Flux Configuration Mapping

> This document validates the consistency between `platform/model/topologies/*.yaml` and `infra/gitops/flux/`.

## k3s-staging Topology

### Deployables Mapping

| Topology Deployable | Flux App Config | Status |
|--------------------|-----------------|--------|
| `api-server` | `infra/gitops/flux/apps/api.yaml` | ✅ |
| `web-bff` | `infra/gitops/flux/apps/web.yaml` | ✅ |
| `admin-bff` | `infra/gitops/flux/apps/admin-bff.yaml` | ✅ |
| `edge-gateway` | `infra/gitops/flux/apps/gateway.yaml` | ✅ |
| `outbox-relay-worker` | *(pending)* | ⚠️ TODO |
| `indexer-worker` | *(pending)* | ⚠️ TODO |
| `projector-worker` | *(pending)* | ⚠️ TODO |
| `scheduler-worker` | *(pending)* | ⚠️ TODO |
| `sync-reconciler-worker` | *(pending)* | ⚠️ TODO |

### Resources Mapping

| Topology Resource | Flux Infrastructure | Status |
|------------------|---------------------|--------|
| `turso` (external) | *(not in Flux, external service)* | ✅ |
| `nats` | `infra/gitops/flux/infrastructure/nats.yaml` | ✅ |
| `cache` (dragonfly) | `infra/gitops/flux/infrastructure/valkey.yaml` | ⚠️ TODO (Dragonfly) |
| `observability` | *(pending)* | ⚠️ TODO |

## single-vps Topology

### Deployables Mapping

| Topology Deployable | Docker Compose | Status |
|--------------------|----------------|--------|
| `api-server` | `infra/local/dockerfile.api` | ✅ |
| `web-bff` | `infra/local/docker-compose.yml` | ✅ |
| `admin-bff` | `infra/local/docker-compose.yml` | ✅ |
| `edge-gateway` | `infra/local/docker-compose.yml` | ✅ |
| Workers | `infra/local/docker-compose.yml` | ✅ |

### Resources Mapping

| Topology Resource | Local Implementation | Status |
|------------------|---------------------|--------|
| `turso` | `infra/local/docker-compose.yml` (Turso embedded) | ✅ |
| `nats` | `infra/local/docker-compose.yml` | ✅ |
| `cache` (valkey) | `infra/local/docker-compose.yml` | ✅ |
| `observability` | `infra/local/docker-compose.yml` | ✅ |

## Gaps Identified

1. **Worker Kustomizations Missing**: Workers (`outbox-relay`, `indexer`, `projector`, `scheduler`, `sync-reconciler`) need Flux Kustomization files for k3s deployment.
2. **Dragonfly vs Valkey**: `k3s-staging` topology uses Dragonfly, but Flux config has Valkey. Need to create `dragonfly.yaml` or update topology.
3. **Observability Stack**: OpenObserve/OpenTelemetry components not yet defined in Flux infrastructure.

## Recommendations

- **Short-term**: Create worker Kustomization files for k3s deployment
- **Medium-term**: Decide on Dragonfly vs Valkey for staging, align topology and Flux
- **Long-term**: Add observability stack (OpenObserve, Grafana, etc.) to Flux infrastructure

## Validation Date

- Last validated: 2026-04-13
- Next review: When adding new deployables or changing topology
