use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::models::dto::category_dto::CategoryDto;
use crate::models::dto::comment_dto::CommentDto;

#[derive(Serialize, Deserialize)]
pub struct PostDto {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct PostDetailDto {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub categories: Vec<CategoryDto>,
    pub comments: Vec<CommentDto>,
}

#[derive(Deserialize)]
pub struct CreatePostDto {
    pub title: String,
    pub body: String,
    pub slug: Option<String>,
    pub category_ids: Option<Vec<i32>>,
}

#[derive(Deserialize)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub body: Option<String>,
    pub slug: Option<String>,
    pub category_ids: Option<Vec<i32>>,
} 