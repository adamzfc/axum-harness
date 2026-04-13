//! Web BFF 入口 — 配置加载 + 服务启动。

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use web_bff::{config::Config, create_router, state::BffState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 初始化 tracing
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,web_bff=debug"));
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true),
        )
        .with(filter)
        .init();

    tracing::info!("Starting Web BFF...");

    let config = Config::from_env()?;
    let state = BffState::new(config).await?;
    let port = state.config.server_port;
    let addr = format!("{}:{}", state.config.server_host, port);
    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Web BFF listening on {}", addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { Box::new(e) })?;

    Ok(())
}
