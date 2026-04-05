//! Counter Tauri commands — bridge to CounterService.

use feature_counter::CounterService;
use storage_turso::EmbeddedTurso;
use tauri::Manager;

#[tauri::command]
pub async fn counter_increment(app: tauri::AppHandle) -> Result<i64, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let service = usecases::counter_service::LibSqlCounterService::new(db);
    service.increment().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn counter_decrement(app: tauri::AppHandle) -> Result<i64, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let service = usecases::counter_service::LibSqlCounterService::new(db);
    service.decrement().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn counter_reset(app: tauri::AppHandle) -> Result<i64, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let service = usecases::counter_service::LibSqlCounterService::new(db);
    service.reset().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn counter_get_value(app: tauri::AppHandle) -> Result<i64, String> {
    let db = app.state::<EmbeddedTurso>().inner().clone();
    let service = usecases::counter_service::LibSqlCounterService::new(db);
    service.get_value().await.map_err(|e| e.to_string())
}
