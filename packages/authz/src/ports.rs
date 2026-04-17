//! Authorization port — abstract interface for authorization decisions.

use async_trait::async_trait;

/// Authorization error variants.
#[derive(Debug, thiserror::Error)]
pub enum AuthzError {
    #[error("Permission denied: {user} cannot {relation} {object}")]
    PermissionDenied {
        user: String,
        relation: String,
        object: String,
    },

    #[error("Store error: {0}")]
    StoreError(String),

    #[error("Model error: {0}")]
    ModelError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),
}

/// Authorization tuple key — (user, relation, object).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct AuthzTupleKey {
    pub user: String,
    pub relation: String,
    pub object: String,
}

impl AuthzTupleKey {
    pub fn new(
        user: impl Into<String>,
        relation: impl Into<String>,
        object: impl Into<String>,
    ) -> Self {
        Self {
            user: user.into(),
            relation: relation.into(),
            object: object.into(),
        }
    }
}

/// Full tuple with optional condition (for OpenFGA compatibility).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthzTuple {
    pub key: AuthzTupleKey,
}

/// Authorization port — the contract that all authz adapters must implement.
#[async_trait]
pub trait AuthzPort: Send + Sync {
    /// Check if a user has a relation to an object.
    ///
    /// Returns `Ok(true)` if allowed, `Ok(false)` if denied.
    /// Returns `Err` for infrastructure failures.
    async fn check(&self, user: &str, relation: &str, object: &str) -> Result<bool, AuthzError>;

    /// Write an authorization tuple (grant a relation).
    async fn write_tuple(&self, tuple: &AuthzTupleKey) -> Result<(), AuthzError>;

    /// Delete an authorization tuple (revoke a relation).
    async fn delete_tuple(&self, tuple: &AuthzTupleKey) -> Result<(), AuthzError>;

    /// List all tuples matching a filter.
    async fn list_tuples(
        &self,
        user: Option<&str>,
        relation: Option<&str>,
        object: Option<&str>,
    ) -> Result<Vec<AuthzTuple>, AuthzError>;

    /// Batch check — verify multiple permissions in one call.
    async fn batch_check(
        &self,
        checks: &[(String, String, String)],
    ) -> Result<Vec<bool>, AuthzError> {
        let mut results = Vec::with_capacity(checks.len());
        for (user, relation, object) in checks {
            results.push(self.check(user, relation, object).await?);
        }
        Ok(results)
    }
}
