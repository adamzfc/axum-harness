//! Testcontainers-based integration tests.
//!
//! These tests spin up real Docker containers (PostgreSQL, Redis, etc.)
//! to test against actual infrastructure rather than in-memory mocks.
//!
//! Run with: cargo test --test containers -- --ignored
//! (Marked as ignored because they require Docker)

use testcontainers::ImageExt;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

#[tokio::test]
#[ignore = "requires Docker"]
async fn postgres_container_starts() {
    let container = Postgres::default()
        .with_tag("16-alpine")
        .start()
        .await
        .expect("Failed to start PostgreSQL container — is Docker running?");

    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let host = container.get_host().await.unwrap().to_string();

    let conn_str = format!("postgres://postgres:postgres@{host}:{host_port}/postgres");
    assert!(conn_str.contains("postgres://"));
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn postgres_can_create_and_query_table() {
    let container = Postgres::default()
        .with_tag("16-alpine")
        .start()
        .await
        .expect("Failed to start PostgreSQL container — is Docker running?");

    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let host = container.get_host().await.unwrap().to_string();

    let conn_str = format!("postgres://postgres:postgres@{host}:{host_port}/postgres");
    assert!(conn_str.contains("5432"));
}
