//! Platform — platform capability trait definitions.
//!
//! Defines abstract interfaces for:
//! - Configuration loading
//! - Telemetry/metrics
//! - Clock (for testable time dependencies)

use async_trait::async_trait;

/// Configuration provider trait.
#[async_trait]
pub trait ConfigProvider: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn get_required(&self, key: &str) -> Result<String, String>;
}

/// Telemetry provider trait.
#[async_trait]
pub trait TelemetryProvider: Send + Sync {
    fn record_metric(&self, name: &str, value: f64);
    fn increment_counter(&self, name: &str);
}

/// Clock trait for testable time dependencies.
#[async_trait]
pub trait Clock: Send + Sync {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

/// System clock implementation.
pub struct SystemClock;

#[async_trait]
impl Clock for SystemClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}
