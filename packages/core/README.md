# Core

Pure Rust business logic — domain ports and usecase implementations.

## Structure

```
core/
├── domain/         # Port trait definitions (StoragePort, LibSQLPort, etc.)
├── usecases/       # Business logic implementations (CounterService, AgentService, etc.)
├── workspace-hack/ # cargo-hakari unified dependency crate (build time optimization)
└── state/          # ⚠️ Shared state machine / cache strategy (stub)
```

## Design

### Domain Layer

Defines **what** the system needs (traits), not **how** it's done:

```rust
pub trait LibSqlPort: Send + Sync {
    async fn execute(&self, sql: &str, params: Vec<String>) -> Result<(), LibsqlError>;
    async fn query<T: DeserializeOwned>(&self, sql: &str, params: Vec<String>) -> Result<Vec<T>, LibsqlError>;
}
```

### Usecases Layer

Implements **how** by composing domain ports:

```rust
pub struct LibSqlCounterService<P: LibSqlPort> {
    port: P,
}

impl<P: LibSqlPort> CounterService for LibSqlCounterService<P> {
    async fn increment(&self) -> Result<i64, CounterError> { ... }
}
```

### Key Rule

**Usecases depend on traits, not concrete implementations.** This means:
- Swap Turso → SurrealDB → Postgres by changing the adapter
- UseCase code is unchanged
- Full unit test coverage without a real database
