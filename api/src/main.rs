use std::error::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use url_shortener::{build_state, config::AppConfig, create_router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting URL Shortener API");

    let config = AppConfig::from_env()?;
    tracing::info!("Configuration loaded: {:?}", config);

    let state = build_state(config.clone()).await?;
    let app = create_router(state);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
