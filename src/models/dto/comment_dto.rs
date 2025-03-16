use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommentDto {
    pub id: i32,
    pub post_id: i32,
    pub user_id: Option<i32>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateCommentDto {
    pub post_id: i32,
    pub user_id: Option<i32>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateCommentDto {
    pub content: Option<String>,
} 