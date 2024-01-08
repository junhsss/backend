use crate::api::handlers;
use axum::routing::{get, post};
use axum::Router;

pub fn configure(backend: aws_sdk_dynamodb::Client) -> Router {
    Router::new()
        .route("/posts", post(handlers::posts::create))
        .route("/posts/:id", get(handlers::posts::find))
        .with_state(backend)
}
