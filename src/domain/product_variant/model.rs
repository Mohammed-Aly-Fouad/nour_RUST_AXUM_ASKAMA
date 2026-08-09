use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct ProductVariant {
    pub id: i64,
    pub product_id: i64,
    pub brand_id: i64,
    pub name: String,
    pub name_ar: String,
    pub sku: String,
    pub barcode: Option<String>,
    pub shelf_location: Option<String>,
    pub stock_quantity: i32,
    pub reorder_threshold: i32,
    pub is_active: bool,
    pub attr: serde_json::Value,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}