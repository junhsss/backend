use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub html: String,
    pub json: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub subscribed: bool,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
