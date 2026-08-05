use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct Product {
    id: Option<i32>,
    name: String,
    name_ar: String,
    parent_id: Option<i32>,
    category_id: i32,
    notes: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,

}