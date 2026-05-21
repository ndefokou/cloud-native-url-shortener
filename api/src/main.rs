use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json, Redirect},
    routing::{get, post},
    Router,
};
use chrono::Utc;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod metrics;

use config::AppConfig;
use error::AppError;
use metrics::Metrics;

// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    redis: Arc<redis::Client>,
    config: Arc<AppConfig>,
    metrics: Arc<Metrics>,
}

// Request/Response structures
#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    url: String,
}

#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    short_code: String,
    short_url: String,
    original_url: String,
    created_at: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    status: String,
    redis: String,
    timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    short_code: String,
    original_url: String,
    clicks: u64,
    created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
    code: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting URL Shortener API");

    // Load configuration
    let config = AppConfig::from_env()?;
    tracing::info!("Configuration loaded: {:?}", config);

    // Initialize Redis client
    let redis_client = redis::Client::open(config.redis_url.clone())?;
    tracing::info!("Redis client initialized");

    // Initialize metrics
    let metrics = Metrics::new()?;

    // Create application state
    let state = AppState {
        redis: Arc::new(redis_client),
        config: Arc::new(config.clone()),
        metrics: Arc::new(metrics),
    };

    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/shorten", post(shorten_url))
        .route("/:code", get(redirect_url))
        .route("/:code/stats", get(get_stats))
        .route("/metrics", get(get_metrics))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive());

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Server listening on {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Health check endpoint
async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    let redis_status = match state.redis.get_connection() {
        Ok(_) => "healthy".to_string(),
        Err(_) => "unhealthy".to_string(),
    };

    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        redis: redis_status,
        timestamp: Utc::now().to_rfc3339(),
    }))
}

// Create short URL endpoint
async fn shorten_url(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, AppError> {
    // Validate URL
    if payload.url.is_empty() {
        return Err(AppError::BadRequest("URL cannot be empty".to_string()));
    }

    let parsed_url = url::Url::parse(&payload.url)
        .map_err(|_| AppError::BadRequest("Invalid URL format".to_string()))?;

    if !["http", "https"].contains(&parsed_url.scheme()) {
        return Err(AppError::BadRequest(
            "URL must use http or https scheme".to_string(),
        ));
    }

    // Generate short code
    let short_code = nanoid::nanoid!(6);
    let created_at = Utc::now().to_rfc3339();

    // Store in Redis
    let mut conn = state.redis.get_connection()?;

    // Store URL mapping
    let url_key = format!("url:{}", short_code);
    let _: () = conn.hset(&url_key, "original_url", &payload.url)?;
    let _: () = conn.hset(&url_key, "created_at", &created_at)?;
    let _: () = conn.hset(&url_key, "short_code", &short_code)?;

    // Initialize click counter
    let clicks_key = format!("clicks:{}", short_code);
    let _: () = conn.set(&clicks_key, 0)?;

    // Set expiration if configured
    if let Some(ttl) = state.config.url_ttl {
        let _: () = conn.expire(&url_key, ttl as usize)?;
        let _: () = conn.expire(&clicks_key, ttl as usize)?;
    }

    tracing::info!("Created short URL: {} -> {}", short_code, payload.url);

    // Update metrics
    state.metrics.urls_created.inc();

    let short_url = format!("{}/{}", state.config.base_url, short_code);

    Ok(Json(ShortenResponse {
        short_code: short_code.clone(),
        short_url,
        original_url: payload.url,
        created_at,
    }))
}

// Redirect to original URL endpoint
async fn redirect_url(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.redis.get_connection()?;

    // Get URL mapping
    let url_key = format!("url:{}", code);

    let original_url: Option<String> = conn.hget(&url_key, "original_url")?;

    match original_url {
        Some(url) => {
            // Increment click counter
            let clicks_key = format!("clicks:{}", code);
            let _: () = conn.incr(&clicks_key, 1)?;

            // Update metrics
            state.metrics.redirects_served.inc();

            tracing::info!("Redirecting {} -> {}", code, url);

            Ok(Redirect::permanent(&url))
        }
        None => {
            tracing::warn!("Short code not found: {}", code);
            Err(AppError::NotFound(format!(
                "Short URL '{}' not found",
                code
            )))
        }
    }
}

// Get URL statistics endpoint
async fn get_stats(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<StatsResponse>, AppError> {
    let mut conn = state.redis.get_connection()?;

    let url_key = format!("url:{}", code);

    // Check if URL exists
    let exists: bool = conn.exists(&url_key)?;
    if !exists {
        return Err(AppError::NotFound(format!(
            "Short URL '{}' not found",
            code
        )));
    }

    // Get URL data
    let original_url: String = conn.hget(&url_key, "original_url")?;
    let created_at: String = conn.hget(&url_key, "created_at")?;

    // Get click count
    let clicks_key = format!("clicks:{}", code);
    let clicks: u64 = conn.get(&clicks_key).unwrap_or(0);

    Ok(Json(StatsResponse {
        short_code: code,
        original_url,
        clicks,
        created_at,
    }))
}

// Prometheus metrics endpoint
async fn get_metrics(State(state): State<AppState>) -> Result<String, AppError> {
    state.metrics.render()
}