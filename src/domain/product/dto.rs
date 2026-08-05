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

// ---------------------------------------------------------------------------
// 1.1 Create Prouduct (POST)
// ---------------------------------------------------------------------------

/// DTO الخاص بإنشاء منتج جديد عبر JSON API
#[derive(Debug, Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub name_ar: String,
    pub category_id: i32, // <-- Allows null or omitting the field
    pub notes: Option<String>,
}


impl CreateProductDto {
    /// Validates the input data payload before touching the database
    pub fn validate(&self) -> Result<(), (StatusCode, String)> {
       

        // 1. English Name Validation
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Product name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name.chars().count() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Product name must not exceed 255 characters".to_string(),
            ));
        }

        // 2. Arabic Name Validation
        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic product name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name_ar.chars().count() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic product name must not exceed 255 characters".to_string(),
            ));
        }

        // 3. Optional Notes Validation
        if let Some(ref notes) = self.notes {
            if notes.trim().chars().count() > 300 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Notes must not exceed 300 characters".to_string(),
                ));
            }
        }

        Ok(())
    }
}