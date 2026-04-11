//! BFF 状态 — 组合所有服务依赖并注入 Axum State。
//!
//! Phase 0: 最小状态 — 仅含配置。后续逐步注入 services/ 实例。

use crate::config::Config;

/// Web BFF 应用状态。
///
/// Automatically implements Clone + Send + Sync (all fields are cheaply cloneable).
#[derive(Clone)]
pub struct BffState {
    pub config: Config,
    // Phase 1+: 注入 counter_service, event_bus 等
    // pub counter_service: Arc<dyn CounterService>,
    // pub event_bus: Arc<dyn EventBus>,
}

impl BffState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self { config })
    }
}
