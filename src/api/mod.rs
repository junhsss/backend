use std::time::Duration;

use axum::{
    http::{
        header::{CONTENT_TYPE, COOKIE},
        HeaderValue, Method, StatusCode,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

use crate::settings::FRONTEND_HOST;

mod handlers;
mod v1;

pub fn configure(backend: aws_sdk_dynamodb::Client) -> Router {
    let origins = [
        "http://localhost:3000".parse().unwrap(),
        "https://frontend-junhsss.vercel.app".parse().unwrap()
    ]
    
    let cors_layer = CorsLayer::new()
        .allow_origin(origins)
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
