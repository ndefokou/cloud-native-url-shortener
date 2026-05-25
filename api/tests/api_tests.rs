use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;
use url_shortener::{
    build_state,
    config::AppConfig,
    create_router,
};

fn test_config() -> AppConfig {
    AppConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        redis_url: std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
        base_url: std::env::var("BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string()),
        url_ttl: None,
    }
}

async fn test_app() -> axum::Router {
    let config = test_config();
    let state = build_state(config)
        .await
        .expect("failed to build app state — is Redis running?");
    create_router(state)
}

#[tokio::test]
async fn health_returns_ok() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn shorten_and_redirect_round_trip() {
    let app = test_app().await;

    let shorten_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/shorten")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({ "url": "https://example.com/page" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(shorten_response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(shorten_response.into_body())
        .await
        .unwrap();
    let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let short_code = payload["short_code"].as_str().unwrap();

    let redirect_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/{}", short_code))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(redirect_response.status(), StatusCode::PERMANENT_REDIRECT);
    assert_eq!(
        redirect_response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap(),
        "https://example.com/page"
    );
}

#[tokio::test]
async fn shorten_rejects_invalid_url() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/shorten")
                .header("content-type", "application/json")
                .body(Body::from(json!({ "url": "not-a-url" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn redirect_unknown_code_returns_not_found() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/unknown")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
