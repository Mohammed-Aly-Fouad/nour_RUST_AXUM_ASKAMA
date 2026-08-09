use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domain::category::dto::CategoryResponseDto;

// ---------------------------------------------------------------------------
// Helper: Convert empty or whitespace-only strings to None during Deserialization
// ---------------------------------------------------------------------------
fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}

// ============================================================================
// SECTION 1: JSON API DTOs
// ============================================================================

/// Shared DTO for displaying product data (used in both the JSON API and Askama templates)
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct ProductResponseDto {
    pub id: i64,
    pub name: String,
    pub name_ar: String,
    pub category_id: i64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 1.1 Create Product (POST API)
// ---------------------------------------------------------------------------

/// DTO الخاص بإنشاء منتج جديد عبر JSON API
#[derive(Debug, Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub name_ar: String,
    pub category_id: i64,
    #[serde(default, deserialize_with = "empty_string_as_none")]
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

// ---------------------------------------------------------------------------
// 1.2 Update Product (PATCH API)
// ---------------------------------------------------------------------------

/// DTO for updating an existing product via PATCH API
#[derive(Debug, Deserialize)]
pub struct UpdateProductDto {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub category_id: Option<i64>,
    /// Nested Option allows passing `null` in JSON to explicitly clear the notes field in the DB
    pub notes: Option<Option<String>>,
}

impl UpdateProductDto {
    /// Validates only the fields that were explicitly supplied in the payload
    pub fn validate(&self) -> Result<(), (StatusCode, String)> {
        // Ensure at least one field is provided for update
        if self.name.is_none()
            && self.name_ar.is_none()
            && self.category_id.is_none()
            && self.notes.is_none()
        {
            return Err((
                StatusCode::BAD_REQUEST,
                "At least one field must be provided to update".to_string(),
            ));
        }

        // 1. English Name Validation (if provided)
        if let Some(ref name) = self.name {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Product name cannot be empty".to_string(),
                ));
            }
            if trimmed.chars().count() > 255 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Product name must not exceed 255 characters".to_string(),
                ));
            }
        }

        // 2. Arabic Name Validation (if provided)
        if let Some(ref name_ar) = self.name_ar {
            let trimmed = name_ar.trim();
            if trimmed.is_empty() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Arabic product name cannot be empty".to_string(),
                ));
            }
            if trimmed.chars().count() > 255 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Arabic product name must not exceed 255 characters".to_string(),
                ));
            }
        }

        // 3. Optional Notes Validation (if provided)
        if let Some(Some(ref notes)) = self.notes {
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

// ============================================================================
// SECTION 2: Web (HTML Forms + Askama Templates)
// ============================================================================

// ---------------------------------------------------------------------------
// 2.1 Create Product (HTML Form)
// ---------------------------------------------------------------------------

/// نموذج إنشاء منتج جديد عبر واجهة الويب (HTML Forms)
#[derive(Debug, Deserialize)]
pub struct CreateProductForm {
    pub name: String,
    pub name_ar: String,
    pub category_id: i64,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

impl CreateProductForm {
    /// يتحقق من صحة بيانات إنشاء المنتج القادمة من الفورم
    pub fn validate(&self) -> Result<(), String> {
        // 1. التحقق من الاسم بالإنجليزية
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err("اسم المنتج بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name.chars().count() > 255 {
            return Err("اسم المنتج بالإنجليزية طويل جداً (255 حرف كحد أقصى)".to_string());
        }

        // 2. التحقق من الاسم بالعربية
        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err("اسم المنتج بالعربية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name_ar.chars().count() > 255 {
            return Err("اسم المنتج بالعربية طويل جداً (255 حرف كحد أقصى)".to_string());
        }

        // 3. التحقق من معرّف القسم (category_id)
        if self.category_id <= 0 {
            return Err("يجب اختيار قسم صالح للمنتج".to_string());
        }

        // 4. التحقق من الملاحظات (إن وجدت)
        if let Some(ref notes) = self.notes {
            if notes.trim().chars().count() > 300 {
                return Err("الملاحظات طويلة جداً (300 حرف كحد أقصى)".to_string());
            }
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 2.2 Update Product (HTML Form)
// ---------------------------------------------------------------------------

/// نموذج تحديث المنتج عبر واجهة الويب (HTML Forms)
#[derive(Debug, Deserialize)]
pub struct UpdateProductForm {
    pub name: String,
    pub name_ar: String,
    pub category_id: i64,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

pub struct MergedProductFormData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
    pub category_id: i64,
    pub notes: Option<&'a str>,
}

impl<'a> MergedProductFormData<'a> {
    /// يتحقق من صحة البيانات النهائية بعد الدمج (نفس رسائل UpdateProductForm)
    pub fn validate(&self) -> Result<(), String> {
        // 1. التحقق من الاسم بالإنجليزية
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err("اسم المنتج بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name.chars().count() > 255 {
            return Err("اسم المنتج بالإنجليزية طويل جداً (255 حرف كحد أقصى)".to_string());
        }

        // 2. التحقق من الاسم بالعربية
        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err("اسم المنتج بالعربية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name_ar.chars().count() > 255 {
            return Err("اسم المنتج بالعربية طويل جداً (255 حرف كحد أقصى)".to_string());
        }

        // 3. التحقق من معرّف القسم
        if self.category_id <= 0 {
            return Err("يجب اختيار قسم صالح للمنتج".to_string());
        }

        // 4. التحقق من الملاحظات (إن وجدت)
        if let Some(notes) = self.notes {
            if notes.trim().chars().count() > 300 {
                return Err("الملاحظات طويلة جداً (300 حرف كحد أقصى)".to_string());
            }
        }

        Ok(())
    }
}

impl UpdateProductForm {
    /// يتحقق من صحة بيانات تحديث المنتج القادمة من الفورم
    pub fn validate(&self) -> Result<(), String> {
        let merged = MergedProductFormData {
            name: &self.name,
            name_ar: &self.name_ar,
            category_id: self.category_id,
            notes: self.notes.as_deref(),
        };
        merged.validate()
    }
}

// ---------------------------------------------------------------------------
// 2.3 Askama Template
// ---------------------------------------------------------------------------

/// قالب الـ Askama لعرض وإدارة المنتجات في صفحات الويب
#[derive(Template, WebTemplate)]
#[template(path = "products.html")]
pub struct ProductsTemplate {
    pub products: Vec<ProductResponseDto>,
    pub categories: Vec<CategoryResponseDto>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub edit_product: Option<ProductResponseDto>,
}

pub mod filters {
    use askama::Values;

    /// يرجّع أول حرف من اسم المنتج (Capital) لعرضه داخل الـ avatar الدائري
    #[askama::filter_fn]
    pub fn first_letter(name: &str, _values: &dyn Values) -> askama::Result<String> {
        Ok(name
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string()))
    }

    /// يولّد لون خلفية ثابت للـ avatar بناءً على اسم المنتج
    #[askama::filter_fn]
    pub fn initial_color(name: &str, _values: &dyn Values) -> askama::Result<String> {
        const PALETTE: [&str; 6] =
            ["#0E7C66", "#2563EB", "#D97706", "#7C3AED", "#DB2777", "#0891B2"];
        let sum: u32 = name.bytes().map(|b| b as u32).sum();
        Ok(PALETTE[sum as usize % PALETTE.len()].to_string())
    }
}