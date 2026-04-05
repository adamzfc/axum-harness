//! Admin Tauri commands — bridge to AdminService.

use feature_admin::{AdminService, DashboardStats};
use storage_turso::EmbeddedTurso;
use tauri::Manager;

#[tauri::command]
pub async fn admin_get_dashboard_stats(app: tauri::AppHandle) -> Result<DashboardStats, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let tenant_service = usecases::tenant_service::LibSqlTenantService::new(db.clone());
    let counter_service = usecases::counter_service::LibSqlCounterService::new(db.clone());
    let admin_service =
        usecases::admin_service::LibSqlAdminService::new(db, tenant_service, counter_service);
    admin_service
        .get_dashboard_stats()
        .await
        .map_err(|e| e.to_string())
}
