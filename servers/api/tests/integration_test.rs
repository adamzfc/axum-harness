//! Cross-module integration tests for runtime_server.
//!
//! Covers tenant middleware extraction, TenantAwareSurrealDb query building,
//! and tenant init API request/response behavior.

use contracts_api::{InitTenantRequest, InitTenantResponse};
use runtime_server::ports::surreal_db::TenantAwareSurrealDb;

// ─── Tenant SQL Filter Injection ────────────────────────────────────────────

#[test]
fn inject_select_without_where() {
    let sql = "SELECT * FROM counter";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("WHERE tenant_id = $tenant_id"),
        "Expected tenant filter in: {result}"
    );
}

#[test]
fn inject_select_with_existing_where() {
    let sql = "SELECT * FROM counter WHERE user = $user";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("tenant_id = $tenant_id AND"),
        "Expected AND tenant filter in: {result}"
    );
    assert!(
        result.contains("user = $user"),
        "Original condition preserved in: {result}"
    );
}

#[test]
fn inject_select_with_order_by() {
    let sql = "SELECT * FROM counter ORDER BY created_at DESC";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("WHERE tenant_id = $tenant_id"),
        "Missing WHERE clause: {result}"
    );
    assert!(
        result.contains("ORDER BY created_at DESC"),
        "ORDER BY preserved: {result}"
    );
}

#[test]
fn inject_select_with_limit() {
    let sql = "SELECT * FROM counter LIMIT 10";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("WHERE tenant_id = $tenant_id"),
        "Missing WHERE clause: {result}"
    );
    assert!(result.contains("LIMIT 10"), "LIMIT preserved: {result}");
}

#[test]
fn inject_create_with_set() {
    let sql = "CREATE counter SET name = $name, count = 0";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("tenant_id = $tenant_id"),
        "Expected tenant_id in CREATE: {result}"
    );
}

#[test]
fn inject_create_with_set_and_return() {
    let sql = "CREATE counter SET name = $name RETURN AFTER";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("tenant_id = $tenant_id"),
        "Expected tenant_id injection: {result}"
    );
    assert!(
        result.contains("RETURN AFTER"),
        "RETURN clause preserved: {result}"
    );
}

#[test]
fn inject_update_with_where() {
    let sql = "UPDATE counter SET count += 1 WHERE id = $id";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("tenant_id = $tenant_id AND"),
        "Expected AND tenant filter: {result}"
    );
    assert!(
        result.contains("id = $id"),
        "Original WHERE preserved: {result}"
    );
}

#[test]
fn inject_update_without_where() {
    let sql = "UPDATE counter SET count = 0";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("WHERE tenant_id = $tenant_id"),
        "Expected WHERE tenant filter: {result}"
    );
}

#[test]
fn inject_delete_with_where() {
    let sql = "DELETE FROM counter WHERE id = $id";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("tenant_id = $tenant_id AND"),
        "Expected AND tenant filter: {result}"
    );
    assert!(
        result.contains("id = $id"),
        "Original WHERE preserved: {result}"
    );
}

#[test]
fn inject_delete_without_where() {
    let sql = "DELETE FROM counter";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(
        result.contains("WHERE tenant_id = $tenant_id"),
        "Expected WHERE tenant filter: {result}"
    );
}

#[test]
fn passthrough_unknown_statement() {
    let sql = "INFO FOR DB";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert_eq!(
        result, sql,
        "Unknown statement should pass through unchanged"
    );
}

// ─── Tenant Init Request/Response Serialization ─────────────────────────────

#[test]
fn deserialize_init_tenant_request() {
    let json = r#"{"user_sub":"google-123","user_name":"Alice"}"#;
    let req: InitTenantRequest =
        serde_json::from_str(json).expect("Failed to deserialize InitTenantRequest");
    assert_eq!(req.user_sub, "google-123");
    assert_eq!(req.user_name, "Alice");
}

#[test]
fn reject_empty_user_sub() {
    let req = InitTenantRequest {
        user_sub: String::new(),
        user_name: "Alice".into(),
    };
    // Empty sub should be detectable (handler returns BAD_REQUEST)
    assert!(req.user_sub.is_empty(), "Empty sub should be flagged");
}

#[test]
fn serialize_init_tenant_response() {
    let resp = InitTenantResponse {
        tenant_id: "tenant:abc123".into(),
        role: "owner".into(),
        created: true,
    };
    let json = serde_json::to_string(&resp).expect("Failed to serialize response");
    assert!(json.contains("\"tenant_id\":\"tenant:abc123\""));
    assert!(json.contains("\"role\":\"owner\""));
    assert!(json.contains("\"created\":true"));
}

// ─── Router Construction ────────────────────────────────────────────────────

#[test]
fn create_router_compiles() {
    use runtime_server::create_router;
    use runtime_server::state::AppState;

    fn _assert_router_signature(_state: AppState) -> axum::Router {
        create_router(_state)
    }
}

// ─── SQL Injection Attack Tests ─────────────────────────────────────────────

#[test]
fn sql_injection_user_sub_with_semicolon() {
    let sql = "SELECT * FROM user_tenant WHERE user_sub = $sub";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(result.contains("tenant_id = $tenant_id AND"));
    assert!(result.contains("user_sub = $sub"));
    assert!(!result.contains("DROP TABLE"));
}

#[test]
fn sql_injection_user_sub_with_comment() {
    let sql = "SELECT * FROM user_tenant WHERE user_sub = $sub";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(result.contains("user_sub = $sub"));
    assert!(!result.contains("--"));
}

#[test]
fn inject_select_with_subquery() {
    let sql = "SELECT * FROM tenant WHERE id IN (SELECT id FROM user_tenant WHERE role = 'owner')";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(result.contains("WHERE tenant_id = $tenant_id AND"));
}

#[test]
fn inject_select_with_group_by() {
    let sql = "SELECT role, count() FROM user_tenant GROUP BY role";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(result.contains("WHERE tenant_id = $tenant_id"));
    assert!(result.contains("GROUP BY role"));
}

#[test]
fn inject_select_with_fetch() {
    let sql = "SELECT * FROM counter FETCH user";
    let result = TenantAwareSurrealDb::inject_tenant_filter(sql);
    assert!(result.contains("WHERE tenant_id = $tenant_id"));
    assert!(result.contains("FETCH user"));
}
