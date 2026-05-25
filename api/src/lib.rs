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

pub mod config;
pub mod error;
pub mod metrics;

use config::AppConfig;
use error::AppError;
use metrics::Metrics;

#[derive(Clone)]
pub struct AppState {
    pub redis: Arc<redis::Client>,
    pub config: Arc<AppConfig>,
    pub metrics: Arc<Metrics>,
}

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub redis: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub short_code: String,
    pub original_url: String,
    pub clicks: u64,
    pub created_at: String,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/shorten", post(shorten_url))
        .route("/:code", get(redirect_url))
        .route("/:code/stats", get(get_stats))
        .route("/metrics", get(get_metrics))
        .with_state(state)
        .layer(tower_http::cors::CorsLayer::permissive())
}

pub async fn build_state(config: AppConfig) -> Result<AppState, Box<dyn std::error::Error>> {
    let redis_client = redis::Client::open(config.redis_url.clone())?;
    let metrics = Metrics::new()?;

    Ok(AppState {
        redis: Arc::new(redis_client),
        config: Arc::new(config),
        metrics: Arc::new(metrics),
    })
}

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

async fn shorten_url(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, AppError> {
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

    let short_code = nanoid::nanoid!(6);
    let created_at = Utc::now().to_rfc3339();

    let mut conn = state.redis.get_connection()?;

    let url_key = format!("url:{}", short_code);
    let _: () = conn.hset(&url_key, "original_url", &payload.url)?;
    let _: () = conn.hset(&url_key, "created_at", &created_at)?;
    let _: () = conn.hset(&url_key, "short_code", &short_code)?;

    let clicks_key = format!("clicks:{}", short_code);
    let _: () = conn.set(&clicks_key, 0)?;

    if let Some(ttl) = state.config.url_ttl {
        let _: () = conn.expire(&url_key, ttl as usize)?;
        let _: () = conn.expire(&clicks_key, ttl as usize)?;
    }

    state.metrics.urls_created.inc();

    let short_url = format!("{}/{}", state.config.base_url, short_code);

    Ok(Json(ShortenResponse {
        short_code: short_code.clone(),
        short_url,
        original_url: payload.url,
        created_at,
    }))
}

async fn redirect_url(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = state.redis.get_connection()?;

    let url_key = format!("url:{}", code);

    let original_url: Option<String> = conn.hget(&url_key, "original_url")?;

    match original_url {
        Some(url) => {
            let clicks_key = format!("clicks:{}", code);
            let _: () = conn.incr(&clicks_key, 1)?;
            state.metrics.redirects_served.inc();
            Ok(Redirect::permanent(&url))
        }
        None => Err(AppError::NotFound(format!(
            "Short URL '{}' not found",
            code
        ))),
    }
}

async fn get_stats(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<StatsResponse>, AppError> {
    let mut conn = state.redis.get_connection()?;

    let url_key = format!("url:{}", code);

    let exists: bool = conn.exists(&url_key)?;
    if !exists {
        return Err(AppError::NotFound(format!(
            "Short URL '{}' not found",
            code
        )));
    }

    let original_url: String = conn.hget(&url_key, "original_url")?;
    let created_at: String = conn.hget(&url_key, "created_at")?;

    let clicks_key = format!("clicks:{}", code);
    let clicks: u64 = conn.get(&clicks_key).unwrap_or(0);

    Ok(Json(StatsResponse {
        short_code: code,
        original_url,
        clicks,
        created_at,
    }))
}

async fn get_metrics(State(state): State<AppState>) -> Result<String, AppError> {
    state.metrics.render()
}
