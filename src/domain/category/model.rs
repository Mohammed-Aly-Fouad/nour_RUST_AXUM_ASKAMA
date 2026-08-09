use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
struct Category {
    pub id: i64,
    pub name: String,
    pub name_ar: String,
    pub parent_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

}