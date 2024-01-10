use argon2::Argon2;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use axum::{extract::State, http::StatusCode, Json};

use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item, to_attribute_value};
use std::collections::HashMap;

use crate::{
    errors::{AppError, DataResponse, Status},
    models::User,
};

#[derive(Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
}

pub async fn signup(
    State(client): State<aws_sdk_dynamodb::Client>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<DataResponse<()>>, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let encrypted_password = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e))?
        .to_string();

    let now = chrono::Utc::now();

    let user: User = User {
        email: payload.email,
        password: encrypted_password,
        subscribed: false,
        updated_at: now,
        created_at: now,
    };

    let item = serde_dynamo::to_item(user).map_err(|e| anyhow::anyhow!(e))?;

    client
        .put_item()
        .table_name("user-staging")
        .set_item(Some(item))
        .send()
        .await?;

    let res = DataResponse {
        status: Status::Success,
        data: None,
    };

    Ok(Json(res))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

pub async fn login(
    State(client): State<aws_sdk_dynamodb::Client>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<DataResponse<()>>, AppError> {
    let key = HashMap::from([(String::from("email"), to_attribute_value(payload.email)?)]);

    let item = client
        .get_item()
        .table_name("user-staging")
        .set_key(Some(key))
        .send()
        .await?;

    let item = match item.item() {
        Some(_item) => _item,
        None => {
            return Err(AppError(
                StatusCode::NOT_FOUND,
                anyhow::anyhow!("Not found"),
            ))
        }
    };

    let user: User = from_item(item.clone())?;

    if validate_password(&payload.password, &user.password).is_err() {
        return Err(AppError(
            StatusCode::UNAUTHORIZED,
            anyhow::anyhow!("Password mismatch"),
        ));
    }

    let res = DataResponse {
        status: Status::Success,
        data: None,
    };

    Ok(Json(res))
}

fn validate_password(password: &str, hash: &str) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!(e))?;
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_e| anyhow::anyhow!("Failed to verify password"))
}
