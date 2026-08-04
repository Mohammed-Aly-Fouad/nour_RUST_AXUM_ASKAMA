use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

// ============================================================================
// SECTION 1: JSON API DTOs
// ============================================================================

/// Shared DTO for displaying brand data (used in both the JSON API and Askama templates)
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct  ProductResponseDto {
    pub id: i32,
    pub name: String,
    pub name_ar: String,
    pub category_id: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
