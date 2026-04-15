//! Counter Tauri commands — bridge to CounterService.

use counter_service::application::TenantScopedCounterService;
use counter_service::domain::CounterId;
use kernel::TenantId;
use storage_turso::EmbeddedTurso;
use tauri::Manager;

fn build_turso_counter_service(
    db: EmbeddedTurso,
) -> TenantScopedCounterService<
    counter_service::infrastructure::LibSqlCounterRepository<EmbeddedTurso>,
> {
    let repo = counter_service::infrastructure::LibSqlCounterRepository::new(db);
    TenantScopedCounterService::new(repo)
}

/// Default tenant for desktop — in production this would come from auth context.
fn desktop_tenant_id() -> TenantId {
    TenantId("desktop-default".into())
}

#[tauri::command]
pub async fn counter_increment(app: tauri::AppHandle) -> Result<i64, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let service = build_turso_counter_service(db);
    let tenant = desktop_tenant_id();
    service
        .increment(&tenant, None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn counter_decrement(app: tauri::AppHandle) -> Result<i64, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let service = build_turso_counter_service(db);
    let tenant = desktop_tenant_id();
    service
        .decrement(&tenant, None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn counter_reset(app: tauri::AppHandle) -> Result<i64, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let service = build_turso_counter_service(db);
    let tenant = desktop_tenant_id();
    service
        .reset(&tenant, None)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn counter_get_value(app: tauri::AppHandle) -> Result<i64, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let service = build_turso_counter_service(db);
    let tenant = desktop_tenant_id();
    service.get_value(&tenant).await.map_err(|e| e.to_string())
}
