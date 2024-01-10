use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

#[derive(Serialize)]
pub struct DataResponse<T> {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub struct AppError(pub StatusCode, pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(ErrorResponse {
                status: Status::Error,
                message: self.1.to_string(),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}
