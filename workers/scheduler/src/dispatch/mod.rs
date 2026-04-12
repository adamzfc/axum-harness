//! Job dispatcher — executes scheduled jobs at their cron intervals.

use async_trait::async_trait;

use crate::jobs::ScheduledJob;

/// Error type for job dispatch.
#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("Job execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Job timeout: {0}")]
    Timeout(String),
}

/// Abstract job executor.
#[async_trait]
pub trait JobExecutor: Send + Sync {
    /// Execute the job.
    async fn execute(&self, job: &ScheduledJob) -> Result<(), DispatchError>;
}

/// Stub executor for testing.
pub struct LoggingExecutor;

#[async_trait]
impl JobExecutor for LoggingExecutor {
    async fn execute(&self, job: &ScheduledJob) -> Result<(), DispatchError> {
        tracing::info!(job_id = %job.id, job_name = %job.name, "executing scheduled job");
        Ok(())
    }
}

/// Dispatches jobs based on cron schedules.
pub struct JobDispatcher<E: JobExecutor> {
    executor: E,
}

impl<E: JobExecutor> JobDispatcher<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    /// Dispatch a single job.
    pub async fn dispatch(&self, job: &ScheduledJob) -> Result<(), DispatchError> {
        self.executor.execute(job).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn dispatch_calls_executor() {
        let executor = LoggingExecutor;
        let dispatcher = JobDispatcher::new(executor);

        let job = ScheduledJob {
            id: "test-job".to_string(),
            name: "Test Job".to_string(),
            cron_expression: "0 * * * *".to_string(),
            enabled: true,
        };

        let result = dispatcher.dispatch(&job).await;
        assert!(result.is_ok());
    }
}
