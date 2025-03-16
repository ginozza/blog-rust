use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::comments;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: Option<i32>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub post_id: i32,
    pub user_id: Option<i32>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = comments)]
pub struct UpdateComment {
    pub content: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
} 