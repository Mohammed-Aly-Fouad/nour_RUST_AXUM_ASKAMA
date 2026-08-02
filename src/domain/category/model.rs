use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
struct Category {
    pub id: Option<i32>,
    pub name: String,
    pub name_ar: String,
    pub parent_id: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

}