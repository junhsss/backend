use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item, to_attribute_value};
use std::collections::HashMap;

use crate::{
    errors::{AppError, DataResponse, Status},
    models::Post,
};

fn generate_random_string(string_length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let mut rng = rand::thread_rng();
    let random_string: String = (0..string_length)
        .map(|_| {
            let index = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect();

    random_string
}

#[derive(Deserialize)]
pub struct PostCreateRequest {
    content: String,
}

#[derive(Serialize)]
pub struct PostCreateResponseData {
    id: String,
    secret: String,
}

pub async fn create(
    State(client): State<aws_sdk_dynamodb::Client>,
    Json(payload): Json<PostCreateRequest>,
) -> Result<Json<DataResponse<PostCreateResponseData>>, AppError> {
    let id = generate_random_string(6);
    let secret = generate_random_string(12);

    let post: Post = Post {
        id: id.clone(),
        secret: secret.clone(),
        content: payload.content,
        private: false,
    };

    let item = serde_dynamo::to_item(post).map_err(|e| anyhow::anyhow!(e))?;

    client
        .put_item()
        .table_name("post-staging")
        .set_item(Some(item))
        .send()
        .await?;

    let res = DataResponse {
        status: Status::Success,
        data: Some(PostCreateResponseData { id, secret }),
    };

    Ok(Json(res))
}

#[derive(Serialize)]
pub struct PostFindResponseData {
    id: String,
    content: String,
}

pub async fn find(
    State(client): State<aws_sdk_dynamodb::Client>,
    Path(id): Path<String>,
) -> Result<Json<DataResponse<PostFindResponseData>>, AppError> {
    let key = HashMap::from([(String::from("id"), to_attribute_value(id)?)]);

    let item = client
        .get_item()
        .table_name("post-staging")
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

    let post: Post = from_item(item.clone())?;

    let res = DataResponse {
        status: Status::Success,
        data: Some(PostFindResponseData {
            id: post.id,
            content: post.content,
        }),
    };

    Ok(Json(res))
}
