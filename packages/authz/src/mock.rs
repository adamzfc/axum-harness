//! Mock authorization adapter — in-memory, suitable for development and testing.
//!
//! This adapter:
//! - Stores tuples in-memory (thread-safe via `tokio::sync::RwLock`)
//! - Supports `check`, `write_tuple`, `delete_tuple`, `list_tuples`
//! - Does NOT support relationship traversal in the mock (direct tuple matching only)
//! - For dev convenience, when the tuple store is empty, `check` returns `true` for all
//!   requests (allow-all mode), matching `dev-secret` JWT mode behavior.

use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::ports::{AuthzError, AuthzPort, AuthzTuple, AuthzTupleKey};

/// In-memory authorization adapter for dev/test.
#[derive(Clone)]
pub struct MockAuthzAdapter {
    tuples: Arc<RwLock<HashSet<AuthzTupleKey>>>,
    /// When true and store is empty, all checks pass (dev convenience).
    allow_all_when_empty: bool,
}

impl MockAuthzAdapter {
    /// Create a new mock adapter with allow-all-when-empty enabled.
    pub fn new() -> Self {
        Self {
            tuples: Arc::new(RwLock::new(HashSet::new())),
            allow_all_when_empty: true,
        }
    }

    /// Create a strict mock that denies when no tuples exist.
    pub fn strict() -> Self {
        Self {
            tuples: Arc::new(RwLock::new(HashSet::new())),
            allow_all_when_empty: false,
        }
    }

    /// Seed the adapter with initial tuples (for testing).
    pub async fn seed(&self, tuples: Vec<AuthzTupleKey>) {
        let mut store = self.tuples.write().await;
        for t in tuples {
            store.insert(t);
        }
    }

    /// Check if the store has any tuples.
    pub async fn is_empty(&self) -> bool {
        self.tuples.read().await.is_empty()
    }

    /// Count of stored tuples.
    pub async fn len(&self) -> usize {
        self.tuples.read().await.len()
    }
}

impl Default for MockAuthzAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthzPort for MockAuthzAdapter {
    async fn check(&self, user: &str, relation: &str, object: &str) -> Result<bool, AuthzError> {
        let store = self.tuples.read().await;

        // Allow-all mode for dev: if store is empty, allow everything
        if self.allow_all_when_empty && store.is_empty() {
            return Ok(true);
        }

        // Direct tuple matching (no relationship traversal in mock)
        let key = AuthzTupleKey::new(user, relation, object);
        Ok(store.contains(&key))
    }

    async fn write_tuple(&self, tuple: &AuthzTupleKey) -> Result<(), AuthzError> {
        let mut store = self.tuples.write().await;
        store.insert(tuple.clone());
        tracing::debug!(
            user = %tuple.user,
            relation = %tuple.relation,
            object = %tuple.object,
            "authz: wrote tuple"
        );
        Ok(())
    }

    async fn delete_tuple(&self, tuple: &AuthzTupleKey) -> Result<(), AuthzError> {
        let mut store = self.tuples.write().await;
        store.remove(tuple);
        tracing::debug!(
            user = %tuple.user,
            relation = %tuple.relation,
            object = %tuple.object,
            "authz: deleted tuple"
        );
        Ok(())
    }

    async fn list_tuples(
        &self,
        user: Option<&str>,
        relation: Option<&str>,
        object: Option<&str>,
    ) -> Result<Vec<AuthzTuple>, AuthzError> {
        let store = self.tuples.read().await;
        let results: Vec<AuthzTuple> = store
            .iter()
            .filter(|key| {
                if let Some(u) = user
                    && key.user != u
                {
                    return false;
                }
                if let Some(r) = relation
                    && key.relation != r
                {
                    return false;
                }
                if let Some(o) = object
                    && key.object != o
                {
                    return false;
                }
                true
            })
            .map(|key| AuthzTuple { key: key.clone() })
            .collect();
        Ok(results)
    }
}
