//! Unit tests for the counter application layer.
//!
//! These tests use an in-memory mock repository to verify that
//! the application service correctly orchestrates repository calls.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use counter_service::application::{RepositoryBackedCounterService, TenantScopedCounterService};
use counter_service::domain::{Counter, CounterId};
use counter_service::ports::{CounterRepository, RepositoryError};
use feature_counter::CounterService;
use kernel::TenantId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// In-memory mock repository for testing.
struct MockCounterRepository {
    counters: Arc<Mutex<HashMap<String, Counter>>>,
}

impl MockCounterRepository {
    fn new() -> Self {
        Self {
            counters: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl CounterRepository for MockCounterRepository {
    async fn load(&self, id: &CounterId) -> Result<Option<Counter>, RepositoryError> {
        let map = self.counters.lock().await;
        Ok(map.get(id.as_str()).cloned())
    }

    async fn increment(&self, id: &CounterId, now: DateTime<Utc>) -> Result<i64, RepositoryError> {
        let mut map = self.counters.lock().await;
        let counter = map
            .entry(id.as_str().to_string())
            .or_insert_with(|| Counter::new(id.clone(), now));
        let updated = counter.clone().increment();
        *map.get_mut(id.as_str()).unwrap() = updated.clone();
        Ok(updated.value)
    }

    async fn decrement(&self, id: &CounterId, now: DateTime<Utc>) -> Result<i64, RepositoryError> {
        let mut map = self.counters.lock().await;
        let counter = map
            .entry(id.as_str().to_string())
            .or_insert_with(|| Counter::new(id.clone(), now));
        let updated = counter.clone().decrement();
        *map.get_mut(id.as_str()).unwrap() = updated.clone();
        Ok(updated.value)
    }

    async fn reset(&self, id: &CounterId, now: DateTime<Utc>) -> Result<(), RepositoryError> {
        let mut map = self.counters.lock().await;
        let counter = Counter::new(id.clone(), now);
        map.insert(id.as_str().to_string(), counter);
        Ok(())
    }

    async fn upsert(&self, counter: &Counter) -> Result<(), RepositoryError> {
        let mut map = self.counters.lock().await;
        map.insert(counter.id.as_str().to_string(), counter.clone());
        Ok(())
    }
}

#[tokio::test]
async fn increment_creates_counter_at_one() {
    let repo = MockCounterRepository::new();
    let service: TenantScopedCounterService<MockCounterRepository> =
        TenantScopedCounterService::new(repo);
    let tenant = TenantId("tenant-a".into());

    let value = service.increment(&tenant).await.unwrap();
    assert_eq!(value, 1, "first increment should produce value 1");
}

#[tokio::test]
async fn increment_is_idempotent_per_tenant() {
    let repo = MockCounterRepository::new();
    let service: TenantScopedCounterService<MockCounterRepository> =
        TenantScopedCounterService::new(repo);
    let tenant = TenantId("tenant-a".into());

    let v1 = service.increment(&tenant).await.unwrap();
    let v2 = service.increment(&tenant).await.unwrap();
    let v3 = service.increment(&tenant).await.unwrap();

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
    assert_eq!(v3, 3);
}

#[tokio::test]
async fn tenant_a_increment_does_not_affect_tenant_b() {
    let repo = MockCounterRepository::new();
    let service: TenantScopedCounterService<MockCounterRepository> =
        TenantScopedCounterService::new(repo);
    let tenant_a = TenantId("tenant-a".into());
    let tenant_b = TenantId("tenant-b".into());

    service.increment(&tenant_a).await.unwrap();
    let b_value = service.get_value(&tenant_b).await.unwrap();

    assert_eq!(b_value, 0, "tenant-b should not see tenant-a's counter");
}

#[tokio::test]
async fn decrement_can_go_negative() {
    let repo = MockCounterRepository::new();
    let service: TenantScopedCounterService<MockCounterRepository> =
        TenantScopedCounterService::new(repo);
    let tenant = TenantId("tenant-a".into());

    let value = service.decrement(&tenant).await.unwrap();
    assert_eq!(value, -1, "decrement on nonexistent counter should be -1");
}

#[tokio::test]
async fn reset_returns_zero() {
    let repo = MockCounterRepository::new();
    let service: TenantScopedCounterService<MockCounterRepository> =
        TenantScopedCounterService::new(repo);
    let tenant = TenantId("tenant-a".into());

    service.increment(&tenant).await.unwrap();
    service.increment(&tenant).await.unwrap();
    let result = service.reset(&tenant).await.unwrap();

    assert_eq!(result, 0, "reset must always return 0");

    let value = service.get_value(&tenant).await.unwrap();
    assert_eq!(value, 0, "counter must be zero after reset");
}

#[tokio::test]
async fn multi_tenant_isolation_after_reset() {
    let repo = MockCounterRepository::new();
    let service: TenantScopedCounterService<MockCounterRepository> =
        TenantScopedCounterService::new(repo);
    let tenant_a = TenantId("tenant-a".into());
    let tenant_b = TenantId("tenant-b".into());

    service.increment(&tenant_a).await.unwrap();
    service.increment(&tenant_a).await.unwrap();
    service.increment(&tenant_b).await.unwrap();

    service.reset(&tenant_a).await.unwrap();

    let a_val = service.get_value(&tenant_a).await.unwrap();
    let b_val = service.get_value(&tenant_b).await.unwrap();

    assert_eq!(a_val, 0, "tenant-a must be zero after reset");
    assert_eq!(b_val, 1, "tenant-b must remain unaffected");
}

#[tokio::test]
async fn get_value_returns_zero_for_missing_counter() {
    let repo = MockCounterRepository::new();
    let service: TenantScopedCounterService<MockCounterRepository> =
        TenantScopedCounterService::new(repo);
    let tenant = TenantId("unknown-tenant".into());

    let value = service.get_value(&tenant).await.unwrap();
    assert_eq!(value, 0, "missing counter should default to zero");
}

#[tokio::test]
async fn repository_backed_service_implements_feature_trait() {
    let repo = MockCounterRepository::new();
    let service: RepositoryBackedCounterService<MockCounterRepository> =
        RepositoryBackedCounterService::new(repo);

    // Verify it implements feature_counter::CounterService
    let _: &dyn feature_counter::CounterService = &service;

    let v = service.increment().await.unwrap();
    assert_eq!(v, 1);
}
