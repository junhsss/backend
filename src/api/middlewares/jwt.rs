use axum::{
    body::Body,
    extract::{Json, Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::errors::ErrorResponse;
use crate::{api::handlers::auth::TokenClaims, errors::Status, settings::AUTH_SECRET};

pub async fn auth(
    State(state): State<aws_sdk_dynamodb::Client>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            auth_value
                .strip_prefix("Bearer ")
                .map(|stripped| stripped.to_owned())
        });

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: Status::Error,
            message: "Missing Bearer token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(AUTH_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| {
        let json_error = ErrorResponse {
            status: Status::Error,
            message: "Invalid bearer token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?
    .claims;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
