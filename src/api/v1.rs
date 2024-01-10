use crate::api::handlers;
use axum::routing::{get, post};
use axum::Router;

pub fn configure(backend: aws_sdk_dynamodb::Client) -> Router {
    Router::new()
        .route("/posts", post(handlers::posts::create))
        .route("/posts/:id", get(handlers::posts::find))
        .route("/auth/signup", post(handlers::auth::signup))
        .route("/auth/login", post(handlers::auth::login))
        .with_state(backend)
}
