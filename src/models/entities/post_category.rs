use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::post_categories;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Associations)]
#[diesel(table_name = post_categories)]
#[diesel(belongs_to(crate::models::entities::post::Post))]
#[diesel(belongs_to(crate::models::entities::category::Category))]
pub struct PostCategory {
    pub id: i32,
    pub post_id: i32,
    pub category_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = post_categories)]
pub struct NewPostCategory {
    pub post_id: i32,
    pub category_id: i32,
} 