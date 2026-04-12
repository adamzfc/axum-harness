# Refactoring Plan: Target Architecture Migration

> **Status**: Draft - Ready for Review  
> **Created**: 2026-04-12  
> **Based on**: `docs/ARCHITECTURE.md` (absolute constitution)  
> **Current State**: Partial implementation with architecture violations  
> **Target State**: Full ARCHITECTURE.md compliance

---

## Executive Summary

This refactoring plan migrates the current codebase from its **partial implementation state** to the **target architecture** defined in `docs/ARCHITECTURE.md`. The plan is organized into **8 phases** with clear handoff points, enabling distributed agent teams to work in parallel where possible and sequentially where necessary.

### Key Principles
1. **Evidence over assumptions**: Every change validated against actual code
2. **Minimal disruption**: No breaking existing functionality during migration
3. **Incremental verification**: Each phase independently verifiable
4. **Rollback safe**: Each phase can be reverted without cascading failures
5. **Platform model first**: `platform/` directory created before infra changes

### Current State vs Target State Gap Analysis

| Dimension | Current State | Target State | Gap |
|-----------|--------------|--------------|-----|
| **Dependency violations** | admin-service → tenant/counter services; user-service → axum | Clean layering | ❌ Must fix |
| **platform/** | Does not exist | Model/schema/generators/validators/catalog | ❌ Missing |
| **workers/** | Does not exist | indexer, outbox-relay, projector, scheduler, sync-reconciler, workflow-runner | ❌ Missing |
| **verification/** | Does not exist | e2e, contract, topology, resilience, performance, golden | ❌ Missing |
| **services/ structure** | Mixed (some complete, some stub) | All services follow domain/application/policies/ports/events/contracts | ⚠️ Partial |
| **servers/ structure** | Mixed (api, web-bff active; admin-bff/mobile-bff stubs) | web-bff, admin-bff, edge-gateway, internal-rpc | ⚠️ Partial |
| **Generated artifacts** | Partial (contracts only) | SDK, platform catalog, K8s manifests, docs | ❌ Missing pipeline |

---

## Phase Dependency Graph

```
Phase 1: Fix dependency violations
    ↓
Phase 2: Create platform/ (model truth source)
    ↓ (parallel)
Phase 3: Create workers/          Phase 5: Restructure servers/
    ↓                                  ↓
Phase 4: Create verification/    Phase 6: Complete services
    ↓                                  ↓
Phase 7: Add platform commands + CI validation
    ↓
Phase 8: Final verification + golden baseline
```

**Critical path**: Phase 1 → Phase 2 → Phase 7 → Phase 8  
**Parallelizable**: Phase 3, 4, 5, 6 can run after Phase 2

---

## Phase 1: Fix Dependency Violations (CRITICAL)

**Duration**: 1-2 days  
**Risk**: HIGH - Breaking existing code  
**Owner**: Senior engineer + code review  
**Handoff**: ✅ Complete when `cargo test -p <service>` passes for all services

### Objectives
1. Remove `admin-service` direct dependency on `tenant-service` and `counter-service`
2. Remove `user-service` dependency on `axum`
3. Ensure all services follow dependency rules from ARCHITECTURE.md §2.2

### Tasks

#### 1.1 Fix admin-service dependency violation
**Problem**: `admin-service` directly depends on `tenant-service` and `counter-service`  
**Solution**: 
- Extract shared contracts/DTOs into `packages/contracts/admin/`
- Define composition interfaces in `packages/features/admin/`
- Move composition logic to `servers/api` or `servers/bff/web-bff` (composition layer)
- Remove cross-service dependencies from `services/admin/Cargo.toml`

**Files to modify**:
- `services/admin-service/Cargo.toml` - remove tenant-service, counter-service deps
- `services/admin-service/src/application/` - refactor to use contracts/ports instead
- `packages/features/admin/` - add composition traits
- `packages/contracts/admin/` - add shared DTOs
- `servers/api/src/routes/admin.rs` - move composition logic here

**Verification**:
```bash
cargo test -p admin-service
cargo tree -p admin-service | grep -E "tenant|counter"  # Should be empty
```

#### 1.2 Fix user-service axum dependency
**Problem**: `user-service` depends on `axum` (HTTP framework in business logic layer)  
**Solution**:
- Remove `axum` from `services/user-service/Cargo.toml`
- Move HTTP-specific types to `servers/api/src/routes/user.rs` or `packages/contracts/api/`
- Ensure user-service only depends on: domain, kernel, contracts_api, features

**Files to modify**:
- `services/user-service/Cargo.toml` - remove axum dependency
- `services/user-service/src/` - remove any axum-specific code
- `servers/api/src/routes/user.rs` - add HTTP layer types here

**Verification**:
```bash
cargo test -p user-service
rg "axum" services/user-service/src/  # Should be empty
```

#### 1.3 Audit all service dependencies
**Problem**: Potential hidden violations  
**Solution**: Run dependency audit across all services

**Verification script**:
```bash
# Check no service depends on another service
for svc in services/*/; do
  svc_name=$(basename "$svc")
  echo "=== $svc_name ==="
  cargo tree -p "$svc_name" | grep "services/" || echo "✅ OK"
done

# Check no service imports concrete adapters
for svc in services/*/; do
  svc_name=$(basename "$svc")
  echo "=== $svc_name adapters check ==="
  cargo tree -p "$svc_name" | grep -E "adapters/(turso|surrealdb|moka)" || echo "✅ OK"
done

# Check no framework imports in domain layer
rg "axum|tauri|hyper|reqwest" services/*/src/domain/  # Should be empty
```

### Acceptance Criteria
- [ ] All services pass `cargo test` independently
- [ ] No cross-service dependencies in Cargo.toml files
- [ ] No framework imports in `services/*/src/domain/`
- [ ] `just boundary-check` passes with zero violations
- [ ] Document remaining violations (if any) in `docs/adr/` with timeline

---

## Phase 2: Create platform/ Directory (TRUTH SOURCE)

**Duration**: 2-3 days  
**Risk**: MEDIUM - New structure, no breaking changes  
**Owner**: Platform team / architect  
**Handoff**: ✅ Complete when `just validate-platform` passes and all models are valid

### Objectives
1. Create complete `platform/` directory structure per ARCHITECTURE.md
2. Define schema for all platform concepts (services, deployables, resources, workflows, topologies, policies)
3. Migrate existing service definitions into `platform/model/services/*.yaml`
4. Create generators and validators
5. Generate `platform/catalog/` as reviewable output

### Tasks

#### 2.1 Create directory structure
```bash
platform/
├── README.md
├── schema/
│   ├── service.schema.json
│   ├── deployable.schema.json
│   ├── resource.schema.json
│   ├── workflow.schema.json
│   ├── topology.schema.json
│   └── policy.schema.json
├── model/
│   ├── services/
│   │   ├── user.yaml
│   │   ├── tenant.yaml
│   │   ├── settings.yaml
│   │   ├── counter.yaml
│   │   ├── admin.yaml
│   │   ├── agent.yaml
│   │   ├── chat.yaml
│   │   └── event-bus.yaml
│   ├── deployables/
│   │   ├── web-bff.yaml
│   │   ├── admin-bff.yaml
│   │   ├── edge-gateway.yaml
│   │   ├── indexer-worker.yaml
│   │   ├── outbox-relay.yaml
│   │   ├── projector.yaml
│   │   ├── scheduler.yaml
│   │   └── sync-reconciler.yaml
│   ├── resources/
│   │   ├── turso.yaml
│   │   ├── nats.yaml
│   │   ├── cache.yaml
│   │   └── observability.yaml
│   ├── workflows/
│   │   └── tenant-onboarding.yaml
│   ├── policies/
│   │   ├── timeout.yaml
│   │   ├── retry.yaml
│   │   ├── outbox.yaml
│   │   └── tenancy.yaml
│   └── topologies/
│       ├── local-dev.yaml
│       └── single-vps.yaml
├── generators/
│   ├── contracts/
│   ├── sdk/
│   ├── compose/
│   └── docs/
├── validators/
│   ├── model-lint/
│   ├── dependency-graph/
│   └── contract-drift/
└── catalog/
    ├── services.generated.yaml
    ├── deployables.generated.yaml
    └── topology.generated.md
```

#### 2.2 Define JSON schemas
Create JSON Schema for each platform concept in `platform/schema/`.

**Example: `service.schema.json`**:
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Service",
  "type": "object",
  "required": ["name", "domain", "ports", "events"],
  "properties": {
    "name": { "type": "string", "pattern": "^[a-z][a-z0-9-]*$" },
    "domain": { "type": "string" },
    "description": { "type": "string" },
    "ports": {
      "type": "array",
      "items": { "$ref": "#/definitions/port" }
    },
    "events": {
      "type": "array",
      "items": { "$ref": "#/definitions/event" }
    },
    "dependencies": {
      "type": "array",
      "items": { "type": "string" }
    }
  },
  "definitions": {
    "port": {
      "type": "object",
      "required": ["name", "type"],
      "properties": {
        "name": { "type": "string" },
        "type": { "enum": ["repository", "cache", "publisher", "clock", "external_api"] }
      }
    },
    "event": {
      "type": "object",
      "required": ["name", "schema"],
      "properties": {
        "name": { "type": "string" },
        "schema": { "type": "string" }
      }
    }
  }
}
```

#### 2.3 Migrate existing services to platform/model/services/
For each existing service in `services/`, create corresponding YAML model.

**Example: `platform/model/services/counter.yaml`**:
```yaml
name: counter
domain: counter
description: Tenant-scoped counter management service
version: 1.0.0

ports:
  - name: counter_repository
    type: repository
    description: Counter state persistence
  - name: event_publisher
    type: publisher
    description: Publish counter events

events:
  - name: CounterIncremented
    schema: packages/contracts/events/counter_incremented.json
  - name: CounterDecremented
    schema: packages/contracts/events/counter_decremented.json
  - name: CounterReset
    schema: packages/contracts/events/counter_reset.json

dependencies:
  - packages/kernel
  - packages/contracts/api
  - packages/features/counter

deployable: counter-service
tests: services/counter-service/tests/
migrations: services/counter-service/migrations/
```

#### 2.4 Create platform validator
Build a Rust-based validator in `platform/validators/model-lint/` that:
- Validates all YAML files against schemas
- Checks dependency graph for cycles
- Ensures all references are resolvable
- Generates validation report

**Verification command**:
```bash
just validate-platform
# Should run:
#   platform/validators/model-lint/validate
#   platform/validators/dependency-graph/check
```

#### 2.5 Create platform generators
Build generators that read `platform/model/` and produce:
- `platform/catalog/services.generated.yaml` - Service registry
- `platform/catalog/deployables.generated.yaml` - Deployment unit registry
- `platform/catalog/topology.generated.md` - Architecture documentation

**Verification**:
```bash
just gen-platform
# Delete catalog/
rm -rf platform/catalog/
# Regenerate
just gen-platform
# Should produce identical output
git diff platform/catalog/  # Should be empty
```

### Acceptance Criteria
- [ ] `platform/` directory exists with complete structure
- [ ] All existing services modeled in `platform/model/services/*.yaml`
- [ ] JSON schemas validate all model files
- [ ] `just validate-platform` passes with zero errors
- [ ] `just gen-platform` generates reproducible catalog
- [ ] Deleting `platform/catalog/` and regenerating produces zero diff

---

## Phase 3: Create workers/ Directory (ASYNC EXECUTION)

**Duration**: 3-4 days  
**Risk**: MEDIUM - New functionality, must integrate with existing services  
**Owner**: Backend team  
**Handoff**: ✅ Complete when all workers compile and pass unit tests

### Objectives
1. Create `workers/` directory structure per ARCHITECTURE.md
2. Migrate async operations from BFF/services into dedicated workers
3. Ensure each worker has: main entry, checkpoint, dedupe, retry strategy
4. Workers must be independently compilable and testable

### Workers to Create

#### 3.1 outbox-relay worker
**Current state**: Outbox pattern exists in `services/event-bus/outbox/` but no dedicated consumer  
**Migration**:
- Create `workers/outbox-relay/`
- Move outbox polling logic from event-bus infrastructure to worker
- Add checkpoint tracking (last processed outbox ID)
- Add deduplication (outbox message ID)
- Add retry strategy (exponential backoff)

**Structure**:
```
workers/outbox-relay/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── polling/
│   │   └── outbox_poller.rs
│   ├── publish/
│   │   └── event_publisher.rs
│   ├── dedupe/
│   │   └── message_dedup.rs
│   └── checkpoint/
│       └── checkpoint_store.rs
├── tests/
└── README.md
```

**Dependencies**:
- `packages/messaging`
- `packages/data/outbox`
- `packages/runtime/ports`
- `services/event-bus` (contracts only)

#### 3.2 indexer worker
**Current state**: `servers/indexer/` exists with minimal structure  
**Migration**:
- Move from `servers/` to `workers/indexer/`
- Implement source/transform/sink pattern
- Add checkpoint per source

**Structure**:
```
workers/indexer/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── sources/
│   │   └── event_source.rs
│   ├── transforms/
│   │   └── document_transformer.rs
│   ├── sinks/
│   │   └── index_sink.rs
│   └── checkpoint/
│       └── source_checkpoint.rs
├── tests/
└── README.md
```

#### 3.3 projector worker
**Purpose**: Build read models from events  
**Structure**:
```
workers/projector/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── consumers/
│   │   └── event_consumer.rs
│   ├── readmodels/
│   │   └── read_model_builder.rs
│   └── checkpoint/
│       └── projection_checkpoint.rs
├── tests/
└── README.md
```

#### 3.4 scheduler worker
**Purpose**: Time-based job dispatch  
**Structure**:
```
workers/scheduler/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── jobs/
│   │   └── job_registry.rs
│   └── dispatch/
│       └── job_dispatcher.rs
├── tests/
└── README.md
```

#### 3.5 sync-reconciler worker
**Purpose**: Sync conflict resolution and reconciliation  
**Structure**:
```
workers/sync-reconciler/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── plans/
│   │   └── reconciliation_plan.rs
│   ├── executors/
│   │   └── reconciliation_executor.rs
│   └── conflict/
│       └── conflict_resolver.rs
├── tests/
└── README.md
```

#### 3.6 workflow-runner worker (optional, phase 3b)
**Purpose**: Execute workflow definitions from `platform/model/workflows/*.yaml`  
**Structure**:
```
workers/workflow-runner/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── engine/
│   │   └── workflow_engine.rs
│   └── steps/
│       └── step_executor.rs
├── tests/
└── README.md
```

### Worker Integration Tasks

#### 3.7 Add workers to Cargo workspace
Update root `Cargo.toml`:
```toml
[workspace]
members = [
  # ... existing members
  "workers/outbox-relay",
  "workers/indexer",
  "workers/projector",
  "workers/scheduler",
  "workers/sync-reconciler",
]
```

#### 3.8 Create worker startup scripts
Add to `justfiles/processes.just`:
```just
dev-workers:
  @echo "Starting workers..."
  cargo run -p outbox-relay-worker &
  cargo run -p indexer-worker &
  cargo run -p projector-worker &
```

#### 3.9 Add worker health checks
Each worker must expose:
- `/healthz` - Liveness probe
- `/readyz` - Readiness probe
- `/metrics` - Prometheus metrics (optional)

### Acceptance Criteria
- [ ] All 5-6 workers compile independently (`cargo build -p <worker>`)
- [ ] Each worker has checkpoint, dedupe, retry logic
- [ ] Workers pass unit tests without starting message systems
- [ ] `cargo test -p <worker>` passes for each worker
- [ ] Workers documented in `platform/model/deployables/*.yaml`
- [ ] Workers added to `Cargo.toml` workspace members

---

## Phase 4: Create verification/ Directory (CROSS-MODULE TESTING)

**Duration**: 2-3 days  
**Risk**: LOW - Additive only, no breaking changes  
**Owner**: QA / Testing team  
**Handoff**: ✅ Complete when verification suite runs end-to-end

### Objectives
1. Create `verification/` directory structure per ARCHITECTURE.md
2. Migrate existing E2E tests from scattered locations
3. Add contract compatibility tests
4. Add topology verification tests
5. Add resilience tests (retry, idempotency, outbox, failover)
6. Create golden baseline for generated artifacts

### Tasks

#### 4.1 Create directory structure
```
verification/
├── e2e/
│   ├── demo-counter/
│   ├── multi-tenant/
│   ├── settings/
│   └── desktop-web-roundtrip/
├── contract/
│   ├── backward-compat/
│   ├── sdk-roundtrip/
│   └── event-schema/
├── topology/
│   ├── single-vps/
│   └── split-workers/
├── resilience/
│   ├── retry/
│   ├── idempotency/
│   ├── outbox/
│   └── failover/
├── performance/
│   ├── bff/
│   ├── gateway/
│   └── cache/
└── golden/
    ├── generated-sdk/
    ├── rendered-manifests/
    └── contracts/
```

#### 4.2 Migrate existing E2E tests
Current E2E tests likely in:
- `apps/web/tests/` (Playwright)
- `apps/desktop/tests/` (Tauri E2E)

Create wrappers in `verification/e2e/` that orchestrate full-stack tests.

**Example: `verification/e2e/demo-counter/e2e.test.ts`**:
```typescript
import { test, expect } from '@playwright/test';

test('counter increment flow', async ({ page }) => {
  // Navigate to counter page
  await page.goto('/counter');
  
  // Get initial value
  const initialText = await page.locator('[data-testid=counter-value]').textContent();
  const initialValue = parseInt(initialText, 10);
  
  // Click increment
  await page.locator('[data-testid=increment-button]').click();
  
  // Wait for update
  await page.waitForSelector('[data-testid=counter-updated]');
  
  // Verify new value
  const newText = await page.locator('[data-testid=counter-value]').textContent();
  const newValue = parseInt(newText, 10);
  
  expect(newValue).toBe(initialValue + 1);
});
```

#### 4.3 Add contract backward compatibility tests
Create test suite that ensures contract changes don't break existing clients.

**Example: `verification/contract/backward-compat/contract.test.ts`**:
```typescript
import { loadContracts } from '../../packages/contracts/generated';
import { validateSchema } from './schema-validator';

test('all HTTP contracts are backward compatible', async () => {
  const contracts = await loadContracts();
  
  for (const contract of contracts.http) {
    const result = validateSchema(contract, contract.previousVersion);
    expect(result.breakingChanges).toHaveLength(0);
  }
});
```

#### 4.4 Add topology verification tests
Tests that verify the system works under different deployment topologies.

**Example: `verification/topology/single-vps/topology.test.ts`**:
```typescript
import { deployTopology } from '../../lib/topology-runner';

test('single-vps topology deploys successfully', async () => {
  const result = await deployTopology('single-vps');
  expect(result.success).toBe(true);
  expect(result.services).toHaveLength(8); // All services running
});
```

#### 4.5 Add resilience tests
Test retry, idempotency, outbox, and failover scenarios.

**Example: `verification/resilience/idempotency/idempotency.test.ts`**:
```typescript
test('duplicate message does not cause duplicate side effects', async () => {
  // Send message twice
  await sendMessage(testMessage);
  await sendMessage(testMessage);
  
  // Verify only one side effect
  const result = await querySideEffect();
  expect(result).toHaveLength(1);
});
```

#### 4.6 Create golden baseline
Generate and commit baseline artifacts for:
- SDK generation output
- Platform catalog output
- Contract generation output

**Verification**:
```bash
just verify-generated
# Compares current generation against golden/ baseline
# Fails if drift detected
```

### Acceptance Criteria
- [ ] `verification/` directory exists with complete structure
- [ ] All E2E tests migrated and passing
- [ ] Contract backward compatibility suite exists
- [ ] Topology verification tests exist
- [ ] Resilience tests cover retry, idempotency, outbox, failover
- [ ] Golden baseline committed to Git
- [ ] `just verify-generated` passes with zero drift

---

## Phase 5: Restructure servers/ (COMPOSITION LAYER)

**Duration**: 2-3 days  
**Risk**: MEDIUM - Server routing changes, may break API  
**Owner**: Backend team  
**Handoff**: ✅ Complete when all servers compile and routes match OpenAPI specs

### Objectives
1. Restructure `servers/` to match target architecture
2. Move composition logic from services to BFFs
3. Create missing servers (admin-bff, internal-rpc)
4. Ensure all servers have OpenAPI specs
5. Remove long-running tasks from BFFs (move to workers)

### Tasks

#### 5.1 Restructure servers/api
**Current state**: Main Axum server aggregating all routes  
**Target**: Keep as internal API server, but remove composition logic

**Actions**:
- Move admin dashboard composition to `servers/bff/admin-bff/`
- Move agent conversation aggregation to `servers/bff/web-bff/` (already there)
- Keep only service-level route registration in `servers/api/`

**Structure**:
```
servers/api/
├── Cargo.toml
├── openapi.yaml
├── src/
│   ├── main.rs
│   ├── middleware/
│   │   ├── auth.rs
│   │   ├── tenant.rs
│   │   ├── cors.rs
│   │   └── telemetry.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── health.rs
│   │   ├── tenant.rs
│   │   ├── counter.rs
│   │   ├── user.rs
│   │   ├── agent.rs
│   │   └── settings.rs
│   └── state.rs
└── README.md
```

#### 5.2 Complete servers/bff/web-bff
**Current state**: Exists but incomplete  
**Actions**:
- Add all routes that mirror `servers/api/`
- Add presentation logic (view models, aggregation)
- Ensure no business logic (only composition)

#### 5.3 Create servers/bff/admin-bff
**Current state**: Stub directory  
**Structure**:
```
servers/bff/admin-bff/
├── Cargo.toml
├── openapi.yaml
├── src/
│   ├── main.rs
│   ├── handlers/
│   │   ├── dashboard.rs
│   │   ├── tenants.rs
│   │   └── metrics.rs
│   ├── presenters/
│   │   └── dashboard_presenter.rs
│   ├── middleware/
│   │   └── admin_auth.rs
│   └── routes/
│       └── mod.rs
└── README.md
```

**Dependencies**:
- Multiple service contracts (tenant, counter, user, agent, settings)
- NO direct service dependencies (use contracts)

#### 5.4 Create servers/bff/mobile-bff (optional, phase 5b)
**Current state**: Stub directory  
**Structure**: Similar to web-bff but mobile-optimized responses

#### 5.5 Restructure servers/gateway
**Current state**: Pingora-based reverse proxy  
**Actions**:
- Ensure routing rules match `platform/model/topologies/local-dev.yaml`
- Add rate limiting middleware
- Add authn/authz middleware (if not already)

#### 5.6 Create servers/internal-rpc (optional, phase 5b)
**Purpose**: Internal gRPC/RPC for service-to-service communication (if needed)

### Acceptance Criteria
- [ ] All servers compile independently
- [ ] All servers have `openapi.yaml` matching actual routes
- [ ] No business logic in any server handler
- [ ] Composition logic moved from services to BFFs
- [ ] Admin-bff exists and aggregates multiple service views
- [ ] `cargo test -p <server>` passes for each server

---

## Phase 6: Complete Missing Service Implementations

**Duration**: 3-4 days  
**Risk**: MEDIUM - Completing stub implementations  
**Owner**: Domain service teams  
**Handoff**: ✅ Complete when all services pass full test suite

### Objectives
1. Complete user-service HTTP implementation
2. Complete chat-service implementation
3. Complete admin-service with ports/infrastructure
4. Ensure all services follow Clean Architecture structure

### Tasks

#### 6.1 Complete user-service HTTP routes
**Current state**: User routes registered but stub  
**Actions**:
- Implement full CRUD handlers in `servers/api/src/routes/user.rs`
- Add user repository port implementation
- Add user service tests

**Files to implement**:
- `servers/api/src/routes/user.rs` - Full route handlers
- `services/user-service/src/infrastructure/repositories/user_repo.rs` - Turso/SQLite implementation
- `services/user-service/tests/integration/user_test.rs` - Integration tests

#### 6.2 Implement chat-service
**Current state**: Stub directory with minimal dependencies  
**Actions**:
- Define chat domain entities (conversation, message, participant)
- Define chat application use cases (send message, list conversations, etc.)
- Define chat ports (message repository, participant repository, event publisher)
- Implement chat infrastructure (Turso/SQLite adapters)
- Add chat contracts (HTTP DTOs, event schemas)
- Add chat Tauri command
- Add chat routes to `servers/api/`

**Structure to create**:
```
services/chat-service/
├── Cargo.toml
├── src/
│   ├── domain/
│   │   ├── entities.rs
│   │   └── value_objects.rs
│   ├── application/
│   │   ├── commands.rs
│   │   ├── queries.rs
│   │   └── chat_service.rs
│   ├── ports/
│   │   ├── message_repository.rs
│   │   ├── participant_repository.rs
│   │   └── event_publisher.rs
│   ├── infrastructure/
│   │   └── repositories/
│   │       ├── message_repo.rs
│   │       └── participant_repo.rs
│   ├── events/
│   │   └── chat_events.rs
│   ├── contracts/
│   │   └── chat_dtos.rs
│   └── lib.rs
├── tests/
├── migrations/
└── README.md
```

#### 6.3 Complete admin-service
**Current state**: Only domain/application, no ports/infrastructure  
**Actions**:
- Add admin ports (dashboard repository, metrics aggregator)
- Add admin infrastructure adapters
- Add admin tests

**Files to add**:
- `services/admin-service/src/ports/` - Port definitions
- `services/admin-service/src/infrastructure/` - Adapter implementations
- `services/admin-service/tests/` - Full test suite

#### 6.4 Audit all services for completeness
Ensure every service has:
- [ ] domain/ with entities and value objects
- [ ] application/ with use cases
- [ ] policies/ (if applicable)
- [ ] ports/ with abstractions
- [ ] infrastructure/ with adapter implementations
- [ ] events/ with event definitions
- [ ] contracts/ with DTOs
- [ ] tests/ with unit and integration tests
- [ ] migrations/ with DB migrations
- [ ] README.md with service documentation

### Acceptance Criteria
- [ ] user-service HTTP routes fully functional
- [ ] chat-service complete implementation with tests
- [ ] admin-service complete with ports/infrastructure
- [ ] All services pass `cargo test`
- [ ] All services documented in `platform/model/services/*.yaml`
- [ ] Tauri commands for chat added

---

## Phase 7: Add Platform Commands + CI Validation

**Duration**: 2 days  
**Risk**: LOW - Tooling only, no breaking changes  
**Owner**: DevX team  
**Handoff**: ✅ Complete when all platform commands work

### Objectives
1. Add all platform validation commands to Justfile
2. Integrate platform validation into CI
3. Add dependency graph validator
4. Add contract drift detector

### Tasks

#### 7.1 Add commands to Justfile
Create `justfiles/platform.just`:
```just
# Platform validation
validate-platform:
  @echo "=== Validating platform models ==="
  cargo run -p platform-validator

# Generate platform artifacts
gen-platform:
  @echo "=== Generating platform catalog ==="
  cargo run -p platform-generator

# Validate dependency graph
validate-deps:
  @echo "=== Validating dependency graph ==="
  cargo run -p dependency-validator

# Validate contracts match platform model
validate-contracts:
  @echo "=== Validating contract drift ==="
  cargo run -p contract-drift-detector

# Verify generated artifacts match golden baseline
verify-generated:
  @echo "=== Verifying generated artifacts ==="
  ./scripts/verify-generated.sh

# Full platform doctor
doctor: validate-platform validate-deps validate-contracts verify-generated
  @echo "=== Platform doctor complete ==="
```

#### 7.2 Add CI integration
Update `.github/workflows/platform.yml`:
```yaml
name: Platform Validation
on:
  push:
    paths:
      - 'platform/**'
      - 'services/**'
      - 'servers/**'
      - 'packages/contracts/**'

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup toolchain
        uses: ./.github/actions/setup
      
      - name: Validate platform models
        run: just validate-platform
      
      - name: Validate dependency graph
        run: just validate-deps
      
      - name: Validate contracts
        run: just validate-contracts
      
      - name: Verify generated artifacts
        run: just verify-generated
```

#### 7.3 Create platform validator crate
`platform/validators/model-lint/`:
```rust
// Validate all YAML files against JSON schemas
// Check dependency graph for cycles
// Ensure all references are resolvable
// Generate validation report
```

#### 7.4 Create contract drift detector
`platform/validators/contract-drift/`:
```rust
// Compare platform/model/services/*.yaml
// Against packages/contracts/
// Detect mismatches in field names, types, endpoints
```

### Acceptance Criteria
- [ ] `just validate-platform` works
- [ ] `just gen-platform` works
- [ ] `just validate-deps` works
- [ ] `just validate-contracts` works
- [ ] `just verify-generated` works
- [ ] `just doctor` runs all validations
- [ ] CI workflow triggers on relevant changes
- [ ] All commands documented in `docs/operations/`

---

## Phase 8: Final Verification + Golden Baseline

**Duration**: 1-2 days  
**Risk**: LOW - Verification only  
**Owner**: QA team + architect  
**Handoff**: ✅ Complete when all checks pass and golden baseline committed

### Objectives
1. Run full verification suite
2. Commit golden baseline for all generated artifacts
3. Document final architecture state
4. Create post-refactoring review

### Tasks

#### 8.1 Run full verification suite
```bash
# Build everything
just build

# Run all tests
just test

# Run E2E tests
just test-e2e-full

# Validate platform
just doctor

# Check contracts
just contracts-check

# Check boundaries
just boundary-check

# Full verification
just verify
```

#### 8.2 Commit golden baseline
```bash
# Generate all artifacts
just gen-platform
just gen-sdk
just gen-contracts

# Copy to golden directory
mkdir -p verification/golden/
cp -r platform/catalog/ verification/golden/generated-platform/
cp -r packages/sdk/typescript/ verification/golden/generated-sdk-ts/
cp -r packages/contracts/generated/ verification/golden/generated-contracts/

# Commit
git add verification/golden/
git commit -m "feat: commit golden baseline after refactoring"
```

#### 8.3 Document final architecture state
Create `docs/architecture/final-state.md`:
- List all services and their completeness status
- List all servers and their routes
- List all workers and their responsibilities
- List all platform models and their validation status
- List all verification suites and their coverage

#### 8.4 Post-refactoring review
Create `docs/adr/009-refactoring-completion.md`:
- What was changed
- Why changes were made
- What remains as technical debt
- Follow-up actions needed
- Metrics before/after (if available)

### Acceptance Criteria
- [ ] `just verify` passes completely
- [ ] Golden baseline committed to `verification/golden/`
- [ ] `docs/architecture/final-state.md` documents current state
- [ ] ADR documents decisions made during refactoring
- [ ] No TODO items left untracked
- [ ] README.md updated to reflect new structure

---

## Parallel Execution Strategy

### Team Allocation
| Team | Phases | Parallel With |
|------|--------|---------------|
| Architect | Phase 2 (platform/) | After Phase 1 |
| Backend Team A | Phase 3 (workers/) | After Phase 2 |
| Backend Team B | Phase 5 (servers/) | After Phase 2 |
| Domain Team | Phase 6 (services) | After Phase 1 |
| QA Team | Phase 4 (verification/) | After Phase 2 |
| DevX Team | Phase 7 (commands/CI) | After Phase 2 |
| All | Phase 8 (final) | After all above |

### Handoff Protocol

Each phase MUST produce:
1. **Phase completion report** in `docs/refactoring/<phase>-completion.md`
2. **Updated task list** marking completed/remaining items
3. **Verification log** showing all acceptance criteria met
4. **Known issues** list for next phase to be aware of

Example handoff document:
```markdown
# Phase X Completion Report

## Status: COMPLETE ✅

## What was done
- List of completed tasks
- Files modified
- Tests added

## Verification
- Commands run
- Results (all passing)

## Known issues
- Issue 1: Description + workaround (if any)
- Issue 2: Low priority, tracked in issue #XXX

## Next phase readiness
- Dependencies resolved: ✅
- Documentation updated: ✅
- Team briefed: ✅
```

---

## Risk Mitigation

### High-Risk Items

| Risk | Impact | Mitigation |
|------|--------|------------|
| Phase 1 breaks existing code | HIGH | Comprehensive test suite before starting; rollback plan ready |
| Admin-service composition complex | MEDIUM | Extract gradually; keep working version at each step |
| Worker checkpoint logic buggy | MEDIUM | Start with in-memory checkpoint; add persistence later |
| Platform model incomplete | LOW | Iterate; models can evolve |
| Generated artifacts drift | MEDIUM | CI checks; automatic drift detection |

### Rollback Strategy
- Each phase is one Git commit (or small series)
- If phase breaks something critical: `git revert <commit>`
- No phase depends on irreversible changes
- Database migrations run forward-only; include down migrations

---

## Success Metrics

### Quantitative
- [ ] Zero dependency violations (`just boundary-check` passes)
- [ ] All 8 services complete with tests
- [ ] All 5 workers operational
- [ ] All 4+ servers with OpenAPI specs
- [ ] Platform models 100% validated
- [ ] E2E test coverage > 80% of user journeys
- [ ] Contract backward compatibility 100%
- [ ] Generated artifacts zero drift

### Qualitative
- [ ] New developer can understand architecture from `platform/` + `docs/`
- [ ] Agent can add new module using `agent/templates/` without guidance
- [ ] CI pipeline completes in < 15 minutes
- [ ] Local dev setup completes in < 5 minutes

---

## Out of Scope (Future Phases)

The following are explicitly **NOT** in this refactoring plan:
- Web3 integration (nostr, farcaster, atproto, evm, ton, solana)
- Wasm plugin system
- Dapr integration
- Multi-cluster Kubernetes
- Production security hardening (SOPS, supply chain)
- Performance optimization
- Mobile app completion
- Browser extension completion

These will be tracked as separate initiatives.

---

## Appendix A: File Change Summary

### Directories to Create
```
platform/                          (new)
workers/                           (new)
verification/                      (new)
servers/bff/admin-bff/             (new)
services/chat-service/             (complete stub → full)
```

### Directories to Modify
```
services/admin-service/            (remove cross-service deps)
services/user-service/             (remove axum dep)
servers/api/                       (clean up composition)
servers/bff/web-bff/               (complete routes)
servers/indexer/                   (move to workers/)
```

### Files to Modify
```
Cargo.toml                         (add workspace members)
Justfile                           (add platform commands)
moon.yml                           (add platform tasks)
AGENTS.md                          (update if needed)
docs/ARCHITECTURE.md               (reference this plan)
```

---

## Appendix B: Command Quick Reference

| Command | Purpose | Phase |
|---------|---------|-------|
| `just validate-platform` | Validate platform models | 2, 7, 8 |
| `just gen-platform` | Generate platform catalog | 2, 7, 8 |
| `just validate-deps` | Check dependency graph | 1, 7, 8 |
| `just validate-contracts` | Detect contract drift | 7, 8 |
| `just verify-generated` | Check artifact drift | 4, 7, 8 |
| `just doctor` | Full platform health | 7, 8 |
| `just boundary-check` | Architecture boundaries | 1, 7, 8 |
| `just verify` | Full verification suite | 8 |

---

## Appendix C: Agent Handoff Template

When handing off work to another agent, copy this template and fill:

```markdown
# Refactoring Handoff: Phase X → Phase Y

## Previous Phase: X
- **Status**: COMPLETE ✅
- **Completion report**: `docs/refactoring/phase-x-completion.md`
- **Git commit**: `<sha>`

## Current Phase: Y
- **Owner**: <team/agent>
- **Start date**: <date>
- **Dependencies**: All resolved ✅

## Context
<brief description of what previous phase delivered>

## What to Do
<detailed task list for this phase>

## Verification
<commands to run to verify work>

## Known Issues
<any issues from previous phase>

## Resources
- `docs/ARCHITECTURE.md` - Constitution
- `docs/REFACTORING_PLAN.md` - This document
- `docs/refactoring/phase-x-completion.md` - Previous phase report
```

---

**END OF PLAN**

This document should be reviewed and approved before any refactoring begins. Each phase is designed to be independently verifiable and rollback-safe.
