//! Reconciliation executor — executes sync plans.

use async_trait::async_trait;

use crate::plans::{ReconciliationPlan, SyncStrategy};
use crate::ReconcilerError;

/// Abstract reconciliation executor.
#[async_trait]
pub trait ReconcileExecutor: Send + Sync {
    /// Execute a reconciliation plan.
    async fn execute(&self, plan: &ReconciliationPlan) -> Result<ReconcileResult, ReconcilerError>;
}

/// Result of a reconciliation execution.
#[derive(Debug, Clone)]
pub struct ReconcileResult {
    pub plan_id: String,
    pub success: bool,
    pub conflicts_found: u32,
    pub conflicts_resolved: u32,
    pub message: String,
}

/// Stub executor for testing.
pub struct StubReconcileExecutor;

#[async_trait]
impl ReconcileExecutor for StubReconcileExecutor {
    async fn execute(&self, plan: &ReconciliationPlan) -> Result<ReconcileResult, ReconcilerError> {
        tracing::info!(plan_id = %plan.id, "executing reconciliation plan (stub)");
        Ok(ReconcileResult {
            plan_id: plan.id.clone(),
            success: true,
            conflicts_found: 0,
            conflicts_resolved: 0,
            message: "Stub execution successful".to_string(),
        })
    }
}

/// Conflict resolution strategy selection.
pub struct ConflictResolver;

impl ConflictResolver {
    /// Resolve a conflict based on the strategy.
    pub fn resolve_conflict(strategy: &SyncStrategy, source_data: &str, target_data: &str) -> String {
        match strategy {
            SyncStrategy::SourceWins => source_data.to_string(),
            SyncStrategy::TargetWins => target_data.to_string(),
            SyncStrategy::LastWriteWins => source_data.to_string(), // Simplified
            SyncStrategy::Manual => format!("CONFLICT: source={source_data}, target={target_data}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conflict_resolver_source_wins() {
        let result = ConflictResolver::resolve_conflict(
            &SyncStrategy::SourceWins,
            "source-data",
            "target-data",
        );
        assert_eq!(result, "source-data");
    }

    #[test]
    fn conflict_resolver_target_wins() {
        let result = ConflictResolver::resolve_conflict(
            &SyncStrategy::TargetWins,
            "source-data",
            "target-data",
        );
        assert_eq!(result, "target-data");
    }
}
