use std::time::Duration;

use axum::{
    http::{
        header::{CONTENT_TYPE, COOKIE},
        Method, StatusCode, HeaderValue, request::Parts,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, AllowOrigin};

mod handlers;
mod middlewares;
mod v1;

pub fn configure(backend: aws_sdk_dynamodb::Client) -> Router {
    let origins = [
        "http://localhost:3000",
        "https://frontend-junhsss.vercel.app",
        "https://writ.ly",
        "https://www.writ.ly",
    ];

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            |origin: &HeaderValue, _request_parts: &Parts| {
                let origin_bytes = origin.as_bytes();
                origin_bytes.ends_with(b".app.github.dev") ||
                origin_bytes == b"http://localhost:3000" ||
                origin_bytes == b"https://frontend-junhsss.vercel.app" ||
                origin_bytes == b"https://writ.ly" ||
                origin_bytes == b"https://www.writ.ly"
            },
        ))
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE, COOKIE])
        .max_age(Duration::from_secs(60 * 60 * 48))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ]);

    Router::new()
        .route("/", get(index))
        .nest("/v1", v1::configure(backend))
        .layer(cors_layer)
        .route("/health-check", get(index))
}

async fn index() -> impl IntoResponse {
    StatusCode::OK
}
