# 🎯 Project Goal (GOAL.md)

> **tauri-sveltekit-axum-moon-template** — A cross-platform, Rust-first microservice boilerplate. Clone via GitHub Template. Ship as a containerized monolith. Scale on demand.

---

## 📌 Project Positioning

This is a **GitHub Template repository**, not a code generator. The intended workflow is:

```
GitHub → "Use this template" → git clone → just setup → just dev up → 🚀
```

The MVP **is already a containerized, orchestratable monolith**. From day one it runs inside Docker Compose, and any business module can be split into an independent service by changing only `Cargo.toml` + deploy config — zero business logic modification.

**Shell scripts are limited to bare VPS bootstrapping** (install `just` + essential modern ops CLI). Everything else — dev, test, build, deploy, migrate — is handled by `Justfile` and `justfiles/*.just` modules.

---

## 🎯 Core Goals

### 1. GitHub Template Ready

- **Goal**: A developer clicks "Use this template" on GitHub and gets a fully working cross-platform app within minutes
- **Acceptance Criteria**:
  - `git clone` + `just setup` + `just dev up` → full stack running (web + Tauri + API)
  - All abstraction layers (`contracts`/`features`/`usecases`/`adapters`) have minimal working examples
  - Every directory has a `README.md` explaining its purpose and usage
  - `docker compose up -f infra/docker/compose.dev.yml up` starts the full environment

### 2. MVP = Containerized Monolith, Ready for Orchestration

- **Goal**: The monolith runs in Docker Compose from day one, and is designed to scale into K8s without rewriting business logic
- **Acceptance Criteria**:
  - **MVP (Day 1)**: Single binary + Docker Compose (Postgres/Redis/NATS as sidecars)
  - **Phase 1 (Scale Prep)**: Each module can `cargo build -p xxx-service` independently
  - **Phase 2 (Microservices)**: In-memory calls → gRPC, single container → K8s Deployments
  - **Phase 3 (Edge/Hybrid)**: Nanocl + Youki + multi-region DR (enable on demand)
- **Core Constraint**: Each evolution step only changes `Cargo.toml` + deploy config, zero business logic modification

### 3. Business Modules Split at Any Time

- **Goal**: Any `services/xxx-service/` can become an independently deployable service at any moment
- **Acceptance Criteria**:
  - `services/` directory already contains the complete skeleton for each business domain
  - Splitting `user-service` into its own repo requires only:
    1. Copy the crate directory
    2. Change `Cargo.toml` workspace path
    3. Update deploy config (Dockerfile / K8s manifest)
  - Zero modification to business logic (`domain/`, `application/`, `interfaces/`)

### 4. Contract-First (Contract-First)

- **Goal**: Frontend and services are decoupled through contracts; refactoring does not affect callers
- **Acceptance Criteria**:
  - `packages/contracts/` is the **single source of truth** for all shared types
  - CI auto-generates TypeScript SDK (`ts-rs`), gRPC SDK (`prost`), OpenAPI SDK (`utoipa`)
  - Frontend/other services depend only on generated SDKs, not on backend implementations
  - Contract changes cause compile-time failures, eliminating runtime interface errors

### 5. Just + xx.just as the Single Command Surface

- **Goal**: Except for bare VPS setup, everything is driven by `just` commands
- **Acceptance Criteria**:
  - **Shell scripts limited to**: `scripts/bootstrap/vps.sh` — installs `just`, `docker`, `git`, and essential ops CLI (`jq`, `yq`, `rg`, `fd`, `mise`)
  - All dev/test/build/deploy/migrate operations are in `justfiles/*.just`:
    - `dev.just` — dev servers, hot reload, Tauri desktop
    - `test.just` — unit, integration, contract, E2E, coverage
    - `quality.just` — format, lint, boundary check, deny
    - `build.just` — workspace build, single-service build, cross-compile
    - `deploy.just` — Docker Compose deploy, systemd deploy, K8s deploy
    - `migrate.just` — DB migrations up/down, status, rollback
  - No `.sh` scripts in `ops/deploy/` or `ops/testing/` — replaced by `just` recipes

### 6. Cross-Platform App

- **Goal**: The template produces Web, Desktop (Tauri), and optionally Mobile/BFF from a single codebase
- **Acceptance Criteria**:
  - `apps/client/web/` — SvelteKit 5, runs in any browser
  - `apps/client/native/` — Tauri v2, builds for macOS/Linux/Windows
  - `apps/client/browser-extension/` — reserved stub, future Chrome/Firefox extension
  - `apps/bff/mobile-bff/` — reserved stub, future mobile API aggregation
  - `apps/bff/admin-bff/` — reserved stub, future admin API aggregation

---

## 🖥️ Core Pages (MVP Scope)

The project focuses on **4+1 core pages**, each corresponding to an independent Feature + UseCase + Route:

| Page | Feature | Domain | Technical Points | Status |
|------|---------|--------|-----------------|--------|
| **Counter** | `feature-counter` | Counter | increment/decrement, optimistic updates, offline sync | ✅ Exists |
| **Admin** | `feature-admin` | Admin Panel | Dashboard stats, user management, system monitoring | ⚠️ stub |
| **Agent** | `feature-agent` | AI Agent | Config management, task execution, result display | ✅ Exists |
| **Chat** | `feature-chat` | Real-time Chat | WebSocket/SSE push, message storage, session management | 🆕 TBD |
| **Settings** | `feature-settings` | User Settings | Preference config, theme switching, privacy settings | 🆕 TBD |

### Page ↔ Technology Stack Mapping

```
apps/client/web/ (SvelteKit)          # Frontend presentation layer
├── routes/(app)/counter/             # Counter page
├── routes/(app)/admin/               # Admin page
├── routes/(app)/agent/               # Agent page
├── routes/(app)/chat/                # Chat page (WebSocket)
└── routes/(app)/settings/            # Settings page

servers/api/ (Axum)                   # Backend API service
├── routes/counter.rs                 # Counter API
├── routes/admin.rs                   # Admin API
├── routes/agent.rs                   # Agent API
├── routes/chat.rs                    # Chat API (WebSocket/SSE)
└── routes/settings.rs                # Settings API

packages/core/usecases/               # Business logic
├── counter_service.rs                # Counter UseCase
├── admin_service.rs                  # Admin UseCase
├── agent_service.rs                  # Agent UseCase
├── chat_service.rs                   # Chat UseCase (TBD)
└── settings_service.rs               # Settings UseCase (TBD)
```

---

## 🏗️ Architecture Design Principles

### Dependency Direction (Non-Violatable)

```
contracts/  ←  Single source of truth for all shared types
    ↑
features/   ←  Defines traits + types, NO implementations, NO dependency on usecases
    ↑
usecases/   ←  Implements traits from features, depends on domain + features + contracts
    ↑
adapters/   ←  External world translation layer, NO business logic
    ↑
apps/ / servers/  ←  Composition layer, NO business logic
```

### Core Rules

1. **Contracts Uniqueness** — Types for the same business concept across layers must align, zero field-level drift
2. **Features No Implementation** — Only traits and types, logic goes into `usecases/`
3. **UseCases Pure Logic** — No dependency on concrete DB/cache/MQ, only on traits
4. **Adapters Thin Translation** — Only protocol translation, business logic stays in `usecases/`
5. **Servers No Logic** — Route → UseCase → Adapter, single responsibility, testable

---

## 🛠️ Technology Stack

### Rust Primary (Core Stack)

| Category | Technology | Maturity | Purpose |
|----------|-----------|----------|---------|
| **HTTP Framework** | Axum 0.8 | ⭐⭐⭐⭐⭐ | API services, BFF layer |
| **Async Runtime** | Tokio 1.50 | ⭐⭐⭐⭐⭐ | All async IO |
| **Database** | sqlx + Turso/libSQL + SurrealDB | ⭐⭐⭐⭐ | Embedded for dev, switchable for prod |
| **Cache** | Moka 0.12 + redis-rs | ⭐⭐⭐⭐⭐ | In-process + distributed cache |
| **Message Queue** | async-nats + rdkafka | ⭐⭐⭐⭐⭐ | Event bus, async communication |
| **ORM/Query** | sqlx | ⭐⭐⭐⭐⭐ | Type-safe SQL |
| **API Docs** | utoipa + utoipa-swagger-ui | ⭐⭐⭐⭐⭐ | OpenAPI 3.1 generation |
| **Type Sync** | ts-rs | ⭐⭐⭐⭐ | Rust → TypeScript type sync |
| **gRPC** | prost + tonic | ⭐⭐⭐⭐⭐ | Inter-service communication |
| **Auth** | jsonwebtoken 10 + OAuth | ⭐⭐⭐⭐ | JWT + third-party login |
| **Logging/Tracing** | tracing + opentelemetry-rust | ⭐⭐⭐⭐⭐ | Structured logging + distributed tracing |
| **Config** | config-rs + figment | ⭐⭐⭐⭐⭐ | Multi-source config (env/file/remote) |
| **Error Handling** | thiserror + anyhow | ⭐⭐⭐⭐⭐ | Library errors + application errors |
| **Validation** | validator | ⭐⭐⭐⭐⭐ | Request parameter validation |

### Go Fallback (Enable on Demand)

| Scenario | Go Solution | Enable Condition |
|----------|------------|-----------------|
| **High-concurrency deduction** | inventory-service (Go) | Flash-sale QPS > 10k |
| **ES Client** | search-service (Go) | ES Go client has higher maturity |
| **K8s Operator** | controller-gen + kubebuilder | When Rust operator ecosystem is insufficient |
| **Specific Middleware** | Official Go SDK | When Rust SDK is immature |

### Frontend Stack

| Category | Technology | Version |
|----------|-----------|---------|
| **Web Framework** | SvelteKit 2 + Svelte 5 (Runes) | Latest |
| **Desktop** | Tauri v2 | 2.10+ |
| **Styling** | Tailwind CSS 4 | Latest |
| **UI Components** | bits-ui | 2.16+ |
| **Type Safety** | ts-rs generated TypeScript | Auto-synced |
| **Testing** | Vitest + Playwright + Testing Library | Unit + E2E |
| **Package Manager** | Bun | 1.3+ |

### Toolchain

| Category | Tool | Purpose |
|----------|------|---------|
| **Task Runner** | just | Cross-platform command runner |
| **Task Orchestration** | moon | Multi-project task dependencies |
| **Tool Management** | mise | Multi-version tool installation |
| **Dependency Unification** | cargo-hakari | Workspace dependency resolution |
| **Code Quality** | cargo-clippy + cargo-deny | Lint + security audit |
| **Testing** | cargo-nextest + cargo-llvm-cov | Test runner + coverage |
| **Load Testing** | drill / lumen | HTTP pressure testing |
| **Container** | Docker Compose | Dev environment |
| **CI/CD** | GitHub Actions | Automated verification |

---

## 📁 Directory Structure (Core Layers)

> Full directory tree: [`docs/ARCHITECTURE.md`](./ARCHITECTURE.md)

```
tauri-sveltekit-axum-moon-template/
├── apps/                           # Application layer (user entry points)
│   ├── client/                     # Client applications
│   │   ├── web/                    # SvelteKit frontend
│   │   └── native/                 # Tauri desktop
│   ├── ops/                        # Ops utility applications
│   └── bff/                        # Backend-for-frontend (enable on demand)
│
├── servers/                        # Server layer (composition layer)
│   ├── api/                        # ✅ Axum API service
│   ├── gateway/                    # ⚠️ API gateway (stub)
│   └── realtime/                   # ⚠️ Realtime service (stub)
│
├── packages/                       # Shared layer (cross-platform reuse)
│   ├── contracts/                  # Contract definitions (single type source)
│   ├── features/                   # Feature definitions (traits + types)
│   ├── core/                       # Core abstractions (domain + usecases)
│   ├── adapters/                   # Adapters (external protocol translators)
│   ├── shared/                     # Shared technical components
│   ├── ui/                         # Frontend UI components
│   └── api-contracts/              # SDK generation (CI auto-publish)
│
├── services/                       # Microservice layer (future independent deployment)
│   ├── user-service/               # User domain
│   ├── agent-service/              # Agent domain
│   ├── chat-service/               # Chat domain
│   ├── counter-service/            # Counter domain
│   └── event-bus/                  # Event bus
│
├── infra/                          # Infrastructure (declarative ops)
│   ├── docker/                     # Docker environment
│   ├── k8s/                        # Kubernetes orchestration
│   ├── terraform/                  # Cloud resource orchestration
│   └── security/                   # Security stack
│
├── ops/                            # Ops layer (executable workflows)
│   ├── migrations/                 # Database migrations
│   ├── observability/              # Observability stack
│   ├── deploy/                     # Deploy workflows (via just, NOT .sh)
│   ├── testing/                    # Specialized tests (via just, NOT .sh)
│   └── backup/                     # Backup
│
├── docs/                           # Knowledge layer (architecture assets)
├── tools/                          # Engineering layer (toolchain)
├── workers/                        # Background tasks (async workers)
├── justfiles/                      # Just command modules
├── scripts/                        # ⚠️ ONLY bootstrap scripts (vps.sh)
└── .github/workflows/              # CI/CD
```

---

## 🔄 Evolutionary Path

### Phase 0: Modular Monolith (Current — MVP)

```
✅ Already in place:
├─ contracts/features/usecases/adapters layered architecture
├─ Counter + Agent pages runnable
├─ Tauri + SvelteKit + Axum technology stack
├─ just + moon + CI/CD toolchain
└─ SurrealDB/Turso storage adapters

⚠️ To complete (short-term):
├─ Admin / Chat / Settings page implementation
├─ packages/shared/ completion
├─ contracts/errors unified error definition
├─ adapters/telemetry telemetry integration
├─ infra/docker/compose.dev.yml (containerized MVP)
└─ docs/architecture ADR documents
```

**Trigger for next phase**: Service change frequency > 3/week OR team > 5 people

       ↓

### Phase 1: Independent Builds (Scale Preparation)

```
🆕 Add:
├─ packages/api-contracts/ contract generation pipeline
├─ services/xxx-service/ each independently buildable
├─ services/event-bus/ event bus + Outbox pattern
├─ infra/docker/compose.dev.yml full environment
├─ ops/observability/ Vector → OpenObserve
└─ apps/bff/ mobile-bff + admin-bff
```

**Trigger for next phase**: Services need independent scaling OR technology stack differentiation

       ↓

### Phase 2: Microservices (Full Orchestration)

```
🆕 Add:
├─ infra/k8s/ Kubernetes deployment configs
├─ infra/terraform/ Cloud resource orchestration
├─ infra/security/ Security stack (Regorus/CrowdSec)
├─ services/ more domain services (order/payment/search)
├─ ops/deploy/ Canary deploy + rollback (via just)
├─ ops/testing/ Contract testing + load testing (via just)
└─ servers/gateway/ Pingora API gateway
```

---

## 🎯 Acceptance Criteria

### Template Readiness

1. **Onboarding** — `git clone` + `just setup` + `just dev up` < 5 min to full stack
2. **Directory Clarity** — Every directory has a `README.md`, clear responsibility, no ambiguity
3. **Minimal Example** — Every abstraction layer has a runnable minimal example (Counter)
4. **Documentation** — ADRs record key decisions, Runbooks record ops workflows

### Evolutionary Architecture

5. **Painless Split** — Extracting `user-service` into its own repo requires only `Cargo.toml` path + deploy config changes, zero business logic modification
6. **Frontend Agnostic** — Frontend depends only on `api-contracts/sdk-gen/typescript`, backend topology changes don't affect frontend
7. **Compile Guarantee** — Contract changes cause compile-time failures, zero runtime interface errors

### Ops Control

8. **One-Command Deploy** — `just deploy-api` deploys the API service
9. **Observability** — `ops/observability/openobserve/` pre-configured business dashboard
10. **Security Compliance** — `cargo-deny` CI mandatory check, JWT signature verification, CORS whitelist

### Efficiency

11. **On-Demand Build** — `just build -p user-service` compiles only changed modules, CI < 5 min
12. **Test Coverage** — Core UseCase unit test coverage > 80%
13. **Contract Testing** — `ops/testing/contract/` ensures backward compatibility

### Just-Only Command Surface

14. **Shell Scripts Limited to VPS Bootstrap** — Only `scripts/bootstrap/vps.sh` exists for bare metal setup
15. **Everything Else via just** — Dev, test, build, deploy, migrate all through `justfiles/*.just`
16. **Cross-Platform Consistency** — `just` recipes work on macOS/Linux/Windows (WSL)

---

## 🚫 Out of Scope

1. **No pursuit of technical novelty** — Prioritize mature, stable Rust/Go ecosystem libraries
2. **No forced microservices** — Modular monolith is the default; microservices are on-demand evolution, not the starting point
3. **No reinventing wheels** — Prioritize integration of existing tools (Vector, OpenObserve, Regorus), not custom-built
4. **No cloud vendor lock-in** — Deploy layer abstracted as Terraform modules, migratable to any cloud
5. **No forced Go usage** — Go is only a fallback when Rust ecosystem is insufficient; Rust-first by default

---

## 📝 Key Decision Records (ADR)

| ADR | Title | Status | Link |
|-----|-------|--------|------|
| 001 | Rust-First + Go Fallback Strategy | ✅ Adopted | `docs/architecture/ADR-001-rust-first.md` |
| 002 | Modular Monolith Evolution Strategy | ✅ Adopted | `docs/architecture/ADR-002-modular-monolith.md` |
| 003 | Just-Only Command Surface (No Shell Scripts) | ✅ Adopted | `docs/architecture/ADR-003-just-only-command-surface.md` |
| 004 | 4+1 Page Scope (MVP) | 📝 Draft | `docs/architecture/ADR-004-mvp-scope.md` |
| 005 | Contract-First SDK Generation Strategy | 📝 Draft | `docs/architecture/ADR-005-contract-first-sdk.md` |
| 006 | Containerized MVP — Ready for Orchestration from Day 1 | 📝 Draft | `docs/architecture/ADR-006-containerized-mvp.md` |

---

## 💡 Core Philosophy

> **Use Rust's type system and ownership model to gain determinism and safety in microservice evolution**.
> Spend 1 hour today defining `Repository trait`, save 100 hours tomorrow refactoring business logic;
> Spend 1 hour today configuring `OpenObserve`, save 100 hours maintaining multiple monitoring systems.
> **Design discipline + Rust ecosystem = evolution freedom + ops cost reduction**.

---

## 📅 Short-Term Action Plan

### Phase 1: Security Hardening (1-2 weeks)

- [ ] Fix JWT signature verification (add JWKS)
- [ ] Configure production CORS whitelist
- [ ] Sensitive info redaction (AgentConfig.api_key)
- [ ] Enable CI workflow (un-disable `.disabled`)

### Phase 2: Architecture Fixes (2-3 weeks)

- [ ] Fix `feature-auth` dependency violation
- [ ] Unify `DashboardStats` type (i64 vs u64)
- [ ] Eliminate `UserProfile` duplicate definitions
- [ ] Complete `packages/shared/` (config/errors/tracing/utils)

### Phase 3: Page Implementation (3-4 weeks)

- [ ] Implement Admin page + UseCase
- [ ] Implement Chat page + WebSocket support
- [ ] Implement Settings page + UseCase
- [ ] Unified error handling component (frontend)

### Phase 4: Template Completion (2-3 weeks)

- [ ] Create `infra/docker/compose.dev.yml` (containerized MVP)
- [ ] Complete `packages/api-contracts/` generation pipeline
- [ ] Write ADR 001~006 documents
- [ ] Add `README.md` to every directory
- [ ] Replace all `.sh` scripts with `justfiles/*.just` recipes

---

> 📌 **This document is the single source of truth for project goals**. Any decision that deviates from these goals must record the reason here and be tracked via a new ADR.
