//! Counter REST API handlers — web-bff version.
//!
//! These handlers use the counter-service implementation via its repository.
//! All responses use contract DTOs from `contracts_api` and `contracts_errors`.

use axum::{
    Json, Router,
    extract::{Extension, State},
    http::StatusCode,
    routing::{get, post},
};
use contracts_api::CounterResponse;
use contracts_errors::{ErrorCode, ErrorResponse};
use counter_service::application::{RepositoryBackedCounterService, TenantScopedCounterService};
use counter_service::contracts::service::{CounterError, CounterService};
use counter_service::domain::CounterId;
use counter_service::infrastructure::LibSqlCounterRepository;
use kernel::TenantId;
use utoipa::OpenApi;

use crate::state::{BffState, DatabaseBackend};

/// Boxed counter service trait object for handler use.
type BoxedCounterService = Box<dyn CounterService + Send + Sync>;

pub fn router() -> Router<BffState> {
    Router::new()
        .route("/api/counter/increment", post(increment))
        .route("/api/counter/decrement", post(decrement))
        .route("/api/counter/reset", post(reset))
        .route("/api/counter/value", get(get_value))
}

/// Increment the tenant's counter value.
#[utoipa::path(
    post,
    path = "/api/counter/increment",
    tag = "counter",
    security(("tenant_auth" = [])),
    responses(
        (status = 200, description = "Counter incremented successfully", body = CounterResponse, content_type = "application/json"),
        (status = 401, description = "Unauthorized — missing tenant context", body = ErrorResponse),
        (status = 409, description = "CAS conflict — concurrent modification", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn increment(
    State(state): State<BffState>,
    tenant: Option<Extension<TenantId>>,
) -> Result<(StatusCode, Json<CounterResponse>), (StatusCode, Json<ErrorResponse>)> {
    let tenant_id = extract_tenant(tenant)?;
    let service = build_service(&state)?;

    let value = service
        .increment(&CounterId::new(tenant_id.as_str()), None)
        .await
        .map_err(map_counter_error)?;

    // Invalidate cache on mutation
    let cache_key = format!("counter:{}", tenant_id.as_str());
    state.counter_cache.invalidate(&cache_key).await;

    Ok((StatusCode::OK, Json(CounterResponse { value })))
}

/// Decrement the tenant's counter value.
#[utoipa::path(
    post,
    path = "/api/counter/decrement",
    tag = "counter",
    security(("tenant_auth" = [])),
    responses(
        (status = 200, description = "Counter decremented successfully", body = CounterResponse, content_type = "application/json"),
        (status = 401, description = "Unauthorized — missing tenant context", body = ErrorResponse),
        (status = 409, description = "CAS conflict — concurrent modification", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn decrement(
    State(state): State<BffState>,
    tenant: Option<Extension<TenantId>>,
) -> Result<(StatusCode, Json<CounterResponse>), (StatusCode, Json<ErrorResponse>)> {
    let tenant_id = extract_tenant(tenant)?;
    let service = build_service(&state)?;

    let value = service
        .decrement(&CounterId::new(tenant_id.as_str()), None)
        .await
        .map_err(map_counter_error)?;

    // Invalidate cache on mutation
    let cache_key = format!("counter:{}", tenant_id.as_str());
    state.counter_cache.invalidate(&cache_key).await;

    Ok((StatusCode::OK, Json(CounterResponse { value })))
}

/// Reset the tenant's counter value to zero.
#[utoipa::path(
    post,
    path = "/api/counter/reset",
    tag = "counter",
    security(("tenant_auth" = [])),
    responses(
        (status = 200, description = "Counter reset successfully", body = CounterResponse, content_type = "application/json"),
        (status = 401, description = "Unauthorized — missing tenant context", body = ErrorResponse),
        (status = 409, description = "CAS conflict — concurrent modification", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn reset(
    State(state): State<BffState>,
    tenant: Option<Extension<TenantId>>,
) -> Result<(StatusCode, Json<CounterResponse>), (StatusCode, Json<ErrorResponse>)> {
    let tenant_id = extract_tenant(tenant)?;
    let service = build_service(&state)?;

    let value = service
        .reset(&CounterId::new(tenant_id.as_str()), None)
        .await
        .map_err(map_counter_error)?;

    // Invalidate cache on mutation
    let cache_key = format!("counter:{}", tenant_id.as_str());
    state.counter_cache.invalidate(&cache_key).await;

    Ok((StatusCode::OK, Json(CounterResponse { value })))
}

/// Get the current counter value for the authenticated tenant.
/// Cache-first: checks cache before hitting the database.
#[utoipa::path(
    get,
    path = "/api/counter/value",
    tag = "counter",
    security(("tenant_auth" = [])),
    responses(
        (status = 200, description = "Current counter value", body = CounterResponse, content_type = "application/json"),
        (status = 401, description = "Unauthorized — missing tenant context", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
)]
async fn get_value(
    State(state): State<BffState>,
    tenant: Option<Extension<TenantId>>,
) -> Result<(StatusCode, Json<CounterResponse>), (StatusCode, Json<ErrorResponse>)> {
    let tenant_id = extract_tenant(tenant)?;
    let cache_key = format!("counter:{}", tenant_id.as_str());

    // Cache-first: check cache before hitting database
    if let Some(cached) = state.counter_cache.get(&cache_key).await {
        return Ok((StatusCode::OK, Json(CounterResponse { value: cached })));
    }

    let service = build_service(&state)?;

    let value = service
        .get_value(&CounterId::new(tenant_id.as_str()))
        .await
        .map_err(map_counter_error)?;

    // Populate cache on read
    state.counter_cache.insert(cache_key.clone(), value).await;

    Ok((StatusCode::OK, Json(CounterResponse { value })))
}

// ── Helpers ──────────────────────────────────────────────────

/// Build a boxed CounterService from the BFF state.
/// Abstracts over embedded and remote database backends.
fn build_service(
    state: &BffState,
) -> Result<BoxedCounterService, (StatusCode, Json<ErrorResponse>)> {
    match state.db.clone() {
        Some(DatabaseBackend::Embedded(db)) => {
            let repo = LibSqlCounterRepository::new(db);
            let service = RepositoryBackedCounterService::new(repo);
            Ok(Box::new(service))
        }
        Some(DatabaseBackend::Remote(db)) => {
            let repo = LibSqlCounterRepository::new(db);
            let service = RepositoryBackedCounterService::new(repo);
            Ok(Box::new(service))
        }
        None => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(
                ErrorCode::InternalError,
                "Embedded database not initialized",
            )),
        )),
    }
}

/// Extract tenant ID from extension, returning proper error response.
fn extract_tenant(
    tenant: Option<Extension<TenantId>>,
) -> Result<TenantId, (StatusCode, Json<ErrorResponse>)> {
    tenant.map(|Extension(id)| id).ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse::new(
                ErrorCode::Unauthorized,
                "Missing tenant context",
            )),
        )
    })
}

/// Map CounterError to HTTP status code and ErrorResponse.
fn map_counter_error(err: CounterError) -> (StatusCode, Json<ErrorResponse>) {
    match err {
        CounterError::CasConflict | CounterError::CasConflictWithDetails { .. } => (
            StatusCode::CONFLICT,
            Json(ErrorResponse::new(
                ErrorCode::Conflict,
                "Counter was modified concurrently",
            )),
        ),
        CounterError::NotFound(msg) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new(ErrorCode::NotFound, &msg)),
        ),
        CounterError::Database(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse::new(
                ErrorCode::DatabaseError,
                format!("Database error: {}", e),
            )),
        ),
    }
}
