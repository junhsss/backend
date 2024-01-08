use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub secret: String,
    pub content: String,
    pub is_private: bool,
}
