//! Job registry — defines and manages scheduled jobs.

use std::collections::HashMap;
use std::sync::RwLock;

/// A scheduled job definition.
#[derive(Debug, Clone)]
pub struct ScheduledJob {
    pub id: String,
    pub name: String,
    pub cron_expression: String,
    pub enabled: bool,
}

/// Registry of all scheduled jobs.
pub struct JobRegistry {
    jobs: RwLock<HashMap<String, ScheduledJob>>,
}

impl JobRegistry {
    pub fn new() -> Self {
        Self {
            jobs: RwLock::new(HashMap::new()),
        }
    }

    /// Register a new job.
    pub async fn register(&self, job: ScheduledJob) {
        self.jobs.write().unwrap().insert(job.id.clone(), job);
    }

    /// Get all enabled jobs.
    pub async fn enabled_jobs(&self) -> Vec<ScheduledJob> {
        self.jobs
            .read()
            .unwrap()
            .values()
            .filter(|j| j.enabled)
            .cloned()
            .collect()
    }

    /// Check if a job exists.
    pub async fn has_job(&self, id: &str) -> bool {
        self.jobs.read().unwrap().contains_key(id)
    }
}

impl Default for JobRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn registry_starts_empty() {
        let registry = JobRegistry::new();
        assert!(!registry.has_job("job-1").await);
    }

    #[tokio::test]
    async fn enabled_jobs_returns_only_enabled() {
        let registry = JobRegistry::new();
        registry
            .register(ScheduledJob {
                id: "job-1".to_string(),
                name: "Enabled Job".to_string(),
                cron_expression: "0 * * * *".to_string(),
                enabled: true,
            })
            .await;
        registry
            .register(ScheduledJob {
                id: "job-2".to_string(),
                name: "Disabled Job".to_string(),
                cron_expression: "0 * * * *".to_string(),
                enabled: false,
            })
            .await;

        let jobs = registry.enabled_jobs().await;
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].id, "job-1");
    }
}
