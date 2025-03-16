use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateCategoryDto {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
} 