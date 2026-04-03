//! Admin service — aggregates real data for dashboard.

use async_trait::async_trait;
use domain::ports::lib_sql::LibSqlPort;
use feature_admin::{AdminError, AdminService, DashboardStats};
use feature_counter::CounterService;

use crate::tenant_service::TenantService;

/// AdminService backed by LibSqlPort + TenantService + CounterService.
pub struct LibSqlAdminService<P: LibSqlPort, T: TenantService, C: CounterService> {
    _port: P,
    tenant_service: T,
    counter_service: C,
}

impl<P: LibSqlPort, T: TenantService, C: CounterService> LibSqlAdminService<P, T, C> {
    pub fn new(port: P, tenant_service: T, counter_service: C) -> Self {
        Self {
            _port: port,
            tenant_service,
            counter_service,
        }
    }
}

#[async_trait]
impl<P: LibSqlPort, T: TenantService, C: CounterService> AdminService
    for LibSqlAdminService<P, T, C>
{
    async fn get_dashboard_stats(&self) -> Result<DashboardStats, AdminError> {
        let tenants =
            self.tenant_service.list_tenants().await.map_err(|e| {
                AdminError::Database(Box::new(std::io::Error::other(e.to_string())))
            })?;
        let counter_value = self
            .counter_service
            .get_value()
            .await
            .map_err(|e| AdminError::Counter(e.to_string()))?;

        Ok(DashboardStats {
            tenant_count: tenants.len() as i64,
            counter_value,
            last_login: tenants.first().map(|t| t.created_at.clone()),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}
