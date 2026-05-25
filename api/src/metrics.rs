use crate::error::AppError;
use lazy_static::lazy_static;
use prometheus::{
    register_counter, register_counter_vec, register_histogram, Counter, CounterVec, Histogram,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetricsError {
    #[error("Failed to register metric: {0}")]
    RegistrationError(#[from] prometheus::Error),
}

lazy_static! {
    // Counter for total URLs created
    pub static ref URLS_CREATED: Counter = register_counter!(
        "url_shortener_urls_created_total",
        "Total number of URLs created"
    ).expect("Failed to register urls_created counter");

    // Counter for total redirects served
    pub static ref REDIRECTS_SERVED: Counter = register_counter!(
        "url_shortener_redirects_total",
        "Total number of redirects served"
    ).expect("Failed to register redirects counter");

    // Counter for HTTP requests by method and path
    pub static ref HTTP_REQUESTS_TOTAL: CounterVec = register_counter_vec!(
        "url_shortener_http_requests_total",
        "Total number of HTTP requests",
        &["method", "path", "status"]
    ).expect("Failed to register http_requests counter");

    // Histogram for request latency
    pub static ref REQUEST_LATENCY: Histogram = register_histogram!(
        "url_shortener_request_latency_seconds",
        "Request latency in seconds",
        vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
    ).expect("Failed to register request_latency histogram");
}

pub struct Metrics {
    pub urls_created: &'static Counter,
    pub redirects_served: &'static Counter,
    pub http_requests_total: &'static CounterVec,
    pub request_latency: &'static Histogram,
}

impl Metrics {
    pub fn new() -> Result<Self, MetricsError> {
        // Initialize lazy_static metrics
        lazy_static::initialize(&URLS_CREATED);
        lazy_static::initialize(&REDIRECTS_SERVED);
        lazy_static::initialize(&HTTP_REQUESTS_TOTAL);
        lazy_static::initialize(&REQUEST_LATENCY);

        Ok(Metrics {
            urls_created: &URLS_CREATED,
            redirects_served: &REDIRECTS_SERVED,
            http_requests_total: &HTTP_REQUESTS_TOTAL,
            request_latency: &REQUEST_LATENCY,
        })
    }

    pub fn render(&self) -> Result<String, AppError> {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = Vec::new();
        encoder
            .encode(&metric_families, &mut buffer)
            .map_err(|e| AppError::Internal(format!("Failed to encode metrics: {}", e)))?;
        String::from_utf8(buffer).map_err(|e| AppError::Internal(format!("Invalid UTF-8: {}", e)))
    }
}
