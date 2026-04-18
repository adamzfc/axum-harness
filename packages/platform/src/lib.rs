//! Platform — platform capability trait definitions.
//!
//! Defines abstract interfaces for:
//! - Configuration loading
//! - Telemetry/metrics
//! - Clock (for testable time dependencies)

use async_trait::async_trait;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::de::DeserializeOwned;

/// Configuration provider trait.
#[async_trait]
pub trait ConfigProvider: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn get_required(&self, key: &str) -> Result<String, String>;
}

/// Shared Figment bootstrap for deployables.
///
/// All runtime entrypoints should load configuration through this helper so the
/// project keeps one stable configuration shape while switching topology.
pub fn load_config<T>(
    defaults: T,
    env_prefix: &str,
    config_file_env: Option<&str>,
) -> Result<T, figment::Error>
where
    T: DeserializeOwned + serde::Serialize,
{
    let mut figment = Figment::new()
        .merge(Serialized::defaults(defaults))
        .merge(Env::prefixed(env_prefix).global());

    if let Some(path) = config_file_env
        .and_then(|key| std::env::var(key).ok())
        .filter(|path| !path.trim().is_empty())
    {
        figment = figment.merge(Toml::file(path));
    }

    figment.extract()
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
