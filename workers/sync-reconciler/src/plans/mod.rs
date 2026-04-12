//! Reconciliation plan — describes what needs to be synced.

/// A reconciliation plan describing what to reconcile.
#[derive(Debug, Clone)]
pub struct ReconciliationPlan {
    pub id: String,
    pub name: String,
    pub source: String,
    pub target: String,
    pub strategy: SyncStrategy,
}

/// Sync strategy — how to reconcile differences.
#[derive(Debug, Clone)]
pub enum SyncStrategy {
    /// Source wins — overwrite target with source data.
    SourceWins,
    /// Target wins — keep target data, ignore source.
    TargetWins,
    /// Last write wins — use timestamps to decide.
    LastWriteWins,
    /// Manual — flag conflicts for human review.
    Manual,
}

impl ReconciliationPlan {
    pub fn new(id: &str, name: &str, source: &str, target: &str, strategy: SyncStrategy) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            source: source.to_string(),
            target: target.to_string(),
            strategy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_creation() {
        let plan = ReconciliationPlan::new(
            "plan-1",
            "Sync users",
            "primary-db",
            "replica-db",
            SyncStrategy::SourceWins,
        );
        assert_eq!(plan.id, "plan-1");
        assert_eq!(plan.source, "primary-db");
    }
}
