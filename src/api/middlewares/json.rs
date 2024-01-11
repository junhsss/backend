use crate::errors::Status;
use axum::{
    async_trait,
    body::Body,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
};
use serde_json::{json, Value};

pub struct CustomJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for CustomJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => Err((
                rejection.status(),
                axum::Json(json!({
                    "status": Status::Error,
                    "message": rejection.body_text(),
                })),
            )),
        }
    }
}
