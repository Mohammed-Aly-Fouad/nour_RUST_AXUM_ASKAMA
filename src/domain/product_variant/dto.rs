use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::domain::brand::dto::BrandResponseDto;
use crate::domain::product::dto::ProductResponseDto;

// ============================================================================
// HELPERS FOR DESERIALIZATION
// ============================================================================

/// يحوّل حقل رقمي فارغ (سلسلة نصية فارغة "") من HTML Form إلى None
pub fn empty_number_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OptionNum<T> {
        Num(T),
        Str(String),
        None,
    }

    match OptionNum::<T>::deserialize(deserializer)? {
        OptionNum::Num(n) => Ok(Some(n)),
        OptionNum::Str(s) if s.trim().is_empty() => Ok(None),
        OptionNum::Str(s) => s.trim().parse::<T>().map(Some).map_err(serde::de::Error::custom),
        OptionNum::None => Ok(None),
    }
}

/// يحوّل حقل نصي فارغ "" من HTML Form إلى None
pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}

// ============================================================================
// SECTION 1: JSON API DTOs
// ============================================================================

#[derive(Deserialize, Serialize, FromRow, Clone, Debug)]
pub struct ProductVariantResponseDto {
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

#[derive(Debug, Deserialize)]
pub struct CreateProductVariantApiDto {
    pub product_id: i64,
    pub brand_id: i64,
    pub name: String,
    pub name_ar: String,
    pub sku: String,
    pub barcode: Option<String>,
    pub shelf_location: Option<String>,
    pub stock_quantity: Option<i32>,
    pub reorder_threshold: Option<i32>,
    pub is_active: Option<bool>,
    pub attr: Option<serde_json::Value>,
    pub notes: Option<String>,
}

impl CreateProductVariantApiDto {
    pub async fn validate(&self, pool: &sqlx::PgPool) -> Result<(), (StatusCode, String)> {
        // 1. التحقق من اسم المتغير بالإنجليزية
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "English name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name.chars().count() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "English name must not exceed 255 characters".to_string(),
            ));
        }

        // 2. التحقق من اسم المتغير بالعربية
        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name_ar.chars().count() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic name must not exceed 255 characters".to_string(),
            ));
        }

        // 3. التحقق من رمز SKU
        let trimmed_sku = self.sku.trim();
        if trimmed_sku.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "SKU is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_sku.chars().count() > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                "SKU must not exceed 100 characters".to_string(),
            ));
        }

        // 4. التحقق من قيود الأعداد (Stock & Reorder Threshold)
        if let Some(sq) = self.stock_quantity {
            if sq < 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Stock quantity cannot be negative".to_string(),
                ));
            }
        }

        if let Some(rt) = self.reorder_threshold {
            if rt < 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Reorder threshold cannot be negative".to_string(),
                ));
            }
        }

        // 5. التحقق من وجود المنتج الأب (Foreign Key: product_id)
        let product_exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM products WHERE id = $1) AS "exists!""#,
            self.product_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if !product_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "Referenced product_id does not exist".to_string(),
            ));
        }

        // 6. التحقق من وجود الماركة (Foreign Key: brand_id)
        let brand_exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM brands WHERE id = $1) AS "exists!""#,
            self.brand_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if !brand_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "Referenced brand_id does not exist".to_string(),
            ));
        }

        // 7. فحص تكرار SKU (Unique Index case-insensitive)
        let sku_exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM product_variants WHERE LOWER(sku) = LOWER($1)) AS "exists!""#,
            trimmed_sku
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if sku_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "SKU already exists".to_string(),
            ));
        }

        // 8. فحص تكرار الباركود (Unique Index)
        if let Some(ref bc) = self.barcode {
            let trimmed_bc = bc.trim();
            if !trimmed_bc.is_empty() {
                let bc_exists = sqlx::query_scalar!(
                    r#"SELECT EXISTS(SELECT 1 FROM product_variants WHERE barcode = $1) AS "exists!""#,
                    trimmed_bc
                )
                .fetch_one(pool)
                .await
                .unwrap_or(false);

                if bc_exists {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        "Barcode already exists".to_string(),
                    ));
                }
            }
        }

        // 9. فحص تكرار الاسم بالإنجليزية لنفس المنتج (Unique per product)
        let name_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM product_variants
                WHERE product_id = $1 AND LOWER(TRIM(name)) = LOWER($2)
            ) AS "exists!"
            "#,
            self.product_id,
            trimmed_name
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if name_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "Variant English name already exists for this product".to_string(),
            ));
        }

        // 10. فحص تكرار الاسم بالعربية لنفس المنتج (Unique per product)
        let name_ar_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM product_variants
                WHERE product_id = $1 AND LOWER(TRIM(name_ar)) = LOWER($2)
            ) AS "exists!"
            "#,
            self.product_id,
            trimmed_name_ar
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if name_ar_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "Variant Arabic name already exists for this product".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductVariantApiDto {
    pub product_id: Option<i64>,
    pub brand_id: Option<i64>,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub shelf_location: Option<String>,
    pub stock_quantity: Option<i32>,
    pub reorder_threshold: Option<i32>,
    pub is_active: Option<bool>,
    pub attr: Option<serde_json::Value>,
    pub notes: Option<String>,
}

pub struct MergedProductVariantData<'a> {
    pub product_id: i64,
    pub brand_id: i64,
    pub name: &'a str,
    pub name_ar: &'a str,
    pub sku: &'a str,
    pub barcode: Option<&'a str>,
    pub stock_quantity: i32,
    pub reorder_threshold: i32,
}

impl<'a> MergedProductVariantData<'a> {
    pub async fn validate(
        &self,
        current_variant_id: i64,
        pool: &sqlx::PgPool,
    ) -> Result<(), (StatusCode, String)> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "الاسم بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string(),
            ));
        }
        if trimmed_name.chars().count() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "اسم المتغير بالإنجليزية يجب ألا يتجاوز 255 حرفاً".to_string(),
            ));
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "الاسم بالعربية مطلوب ولا يمكن تركه فارغاً".to_string(),
            ));
        }
        if trimmed_name_ar.chars().count() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                "اسم المتغير بالعربية يجب ألا يتجاوز 255 حرفاً".to_string(),
            ));
        }

        let trimmed_sku = self.sku.trim();
        if trimmed_sku.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "رمز SKU مطلوب ولا يمكن تركه فارغاً".to_string(),
            ));
        }
        if trimmed_sku.chars().count() > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                "رمز SKU يجب ألا يتجاوز 100 حرف".to_string(),
            ));
        }

        if self.stock_quantity < 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                "كمية المخزون لا يمكن أن تكون بالسالب".to_string(),
            ));
        }

        if self.reorder_threshold < 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                "حد إعادة الطلب لا يمكن أن يكون بالسالب".to_string(),
            ));
        }

        // فحص وجود المنتج
        let product_exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM products WHERE id = $1) AS "exists!""#,
            self.product_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if !product_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "المنتج التابع له غير موجود".to_string(),
            ));
        }

        // فحص وجود الماركة
        let brand_exists = sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM brands WHERE id = $1) AS "exists!""#,
            self.brand_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if !brand_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "الماركة المحددة غير موجودة".to_string(),
            ));
        }

        // فحص تكرار SKU
        let sku_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM product_variants
                WHERE LOWER(sku) = LOWER($1) AND id != $2
            ) AS "exists!"
            "#,
            trimmed_sku,
            current_variant_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if sku_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "رمز SKU موجود بالفعل لمتغير آخر".to_string(),
            ));
        }

        // فحص تكرار الباركود
        if let Some(bc) = self.barcode {
            let trimmed_bc = bc.trim();
            if !trimmed_bc.is_empty() {
                let bc_exists = sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS(
                        SELECT 1 FROM product_variants
                        WHERE barcode = $1 AND id != $2
                    ) AS "exists!"
                    "#,
                    trimmed_bc,
                    current_variant_id
                )
                .fetch_one(pool)
                .await
                .unwrap_or(false);

                if bc_exists {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        "الباركود موجود بالفعل لمتغير آخر".to_string(),
                    ));
                }
            }
        }

        // فحص تكرار الاسم بالإنجليزية للمنتج
        let name_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM product_variants
                WHERE product_id = $1 AND LOWER(TRIM(name)) = LOWER($2) AND id != $3
            ) AS "exists!"
            "#,
            self.product_id,
            trimmed_name,
            current_variant_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if name_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "اسم المتغير بالإنجليزية موجود بالفعل لهذا المنتج".to_string(),
            ));
        }

        // فحص تكرار الاسم بالعربية للمنتج
        let name_ar_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM product_variants
                WHERE product_id = $1 AND LOWER(TRIM(name_ar)) = LOWER($2) AND id != $3
            ) AS "exists!"
            "#,
            self.product_id,
            trimmed_name_ar,
            current_variant_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if name_ar_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "اسم المتغير بالعربية موجود بالفعل لهذا المنتج".to_string(),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// SECTION 2: Web (HTML Forms + Askama Templates)
// ============================================================================

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProductVariantForm {
    pub product_id: i64,
    pub brand_id: i64,
    pub name: String,
    pub name_ar: String,
    pub sku: String,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub barcode: Option<String>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub shelf_location: Option<String>,

    pub stock_quantity: Option<i32>,
    pub reorder_threshold: Option<i32>,

    #[serde(default)]
    pub is_active: Option<bool>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

impl CreateProductVariantForm {
    pub fn validate(&self, existing_variants: &[ProductVariantResponseDto]) -> Result<(), String> {
        let merged = MergedProductVariantFormData {
            product_id: self.product_id,
            brand_id: self.brand_id,
            name: &self.name,
            name_ar: &self.name_ar,
            sku: &self.sku,
            barcode: self.barcode.as_deref(),
            stock_quantity: self.stock_quantity.unwrap_or(0),
            reorder_threshold: self.reorder_threshold.unwrap_or(0),
        };
        merged.validate(None, existing_variants)
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductVariantForm {
    pub product_id: i64,
    pub brand_id: i64,
    pub name: String,
    pub name_ar: String,
    pub sku: String,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub barcode: Option<String>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub shelf_location: Option<String>,

    pub stock_quantity: Option<i32>,
    pub reorder_threshold: Option<i32>,

    #[serde(default)]
    pub is_active: Option<bool>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

impl UpdateProductVariantForm {
    pub fn validate(
        &self,
        current_variant_id: i64,
        existing_variants: &[ProductVariantResponseDto],
    ) -> Result<(), String> {
        let merged = MergedProductVariantFormData {
            product_id: self.product_id,
            brand_id: self.brand_id,
            name: &self.name,
            name_ar: &self.name_ar,
            sku: &self.sku,
            barcode: self.barcode.as_deref(),
            stock_quantity: self.stock_quantity.unwrap_or(0),
            reorder_threshold: self.reorder_threshold.unwrap_or(0),
        };
        merged.validate(Some(current_variant_id), existing_variants)
    }
}

pub struct MergedProductVariantFormData<'a> {
    pub product_id: i64,
    pub brand_id: i64,
    pub name: &'a str,
    pub name_ar: &'a str,
    pub sku: &'a str,
    pub barcode: Option<&'a str>,
    pub stock_quantity: i32,
    pub reorder_threshold: i32,
}

impl<'a> MergedProductVariantFormData<'a> {
    pub fn validate(
        &self,
        current_variant_id: Option<i64>,
        existing_variants: &[ProductVariantResponseDto],
    ) -> Result<(), String> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err("الاسم بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name.chars().count() > 255 {
            return Err("اسم المتغير بالإنجليزية يجب ألا يتجاوز 255 حرفاً".to_string());
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err("الاسم بالعربية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name_ar.chars().count() > 255 {
            return Err("اسم المتغير بالعربية يجب ألا يتجاوز 255 حرفاً".to_string());
        }

        let trimmed_sku = self.sku.trim();
        if trimmed_sku.is_empty() {
            return Err("رمز SKU مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_sku.chars().count() > 100 {
            return Err("رمز SKU يجب ألا يتجاوز 100 حرف".to_string());
        }

        if self.stock_quantity < 0 {
            return Err("كمية المخزون لا يمكن أن تكون بالسالب".to_string());
        }

        if self.reorder_threshold < 0 {
            return Err("حد إعادة الطلب لا يمكن أن يكون بالسالب".to_string());
        }

        // 1. فحص تكرار الـ SKU (In-Memory Case-insensitive)
        let sku_exists = existing_variants.iter().any(|v| {
            Some(v.id) != current_variant_id
                && v.sku.trim().eq_ignore_ascii_case(trimmed_sku)
        });
        if sku_exists {
            return Err("رمز SKU موجود بالفعل لمتغير آخر".to_string());
        }

        // 2. فحص تكرار الباركود
        if let Some(bc) = self.barcode {
            let trimmed_bc = bc.trim();
            if !trimmed_bc.is_empty() {
                let bc_exists = existing_variants.iter().any(|v| {
                    Some(v.id) != current_variant_id
                        && v.barcode.as_deref().map(|s| s.trim()) == Some(trimmed_bc)
                });
                if bc_exists {
                    return Err("الباركود موجود بالفعل لمتغير آخر".to_string());
                }
            }
        }

        // 3. فحص تكرار الاسم بالإنجليزية لنفس المنتج
        let name_exists = existing_variants.iter().any(|v| {
            Some(v.id) != current_variant_id
                && v.product_id == self.product_id
                && v.name.trim().eq_ignore_ascii_case(trimmed_name)
        });
        if name_exists {
            return Err("اسم المتغير بالإنجليزية موجود بالفعل لهذا المنتج".to_string());
        }

        // 4. فحص تكرار الاسم بالعربية لنفس المنتج
        let name_ar_exists = existing_variants.iter().any(|v| {
            Some(v.id) != current_variant_id
                && v.product_id == self.product_id
                && v.name_ar.trim().eq_ignore_ascii_case(trimmed_name_ar)
        });
        if name_ar_exists {
            return Err("اسم المتغير بالعربية موجود بالفعل لهذا المنتج".to_string());
        }

        Ok(())
    }
}

// ============================================================================
// SECTION 3: Askama Template Structs & Helpers
// ============================================================================

#[derive(Debug, Clone)]
pub struct ProductVariantRow {
    pub id: i64,
    pub product_id: i64,
    pub product_name: String,
    pub brand_id: i64,
    pub brand_name: String,
    pub name: String,
    pub name_ar: String,
    pub sku: String,
    pub barcode: Option<String>,
    pub shelf_location: Option<String>,
    pub stock_quantity: i32,
    pub reorder_threshold: i32,
    pub is_active: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl ProductVariantRow {
    pub fn build_rows(
        variants: &[ProductVariantResponseDto],
        products_map: &HashMap<i64, String>,
        brands_map: &HashMap<i64, String>,
    ) -> Vec<ProductVariantRow> {
        variants
            .iter()
            .map(|v| ProductVariantRow {
                id: v.id,
                product_id: v.product_id,
                product_name: products_map
                    .get(&v.product_id)
                    .cloned()
                    .unwrap_or_else(|| "Unknown Product".to_string()),
                brand_id: v.brand_id,
                brand_name: brands_map
                    .get(&v.brand_id)
                    .cloned()
                    .unwrap_or_else(|| "Unknown Brand".to_string()),
                name: v.name.clone(),
                name_ar: v.name_ar.clone(),
                sku: v.sku.clone(),
                barcode: v.barcode.clone(),
                shelf_location: v.shelf_location.clone(),
                stock_quantity: v.stock_quantity,
                reorder_threshold: v.reorder_threshold,
                is_active: v.is_active,
                notes: v.notes.clone(),
                created_at: v.created_at,
            })
            .collect()
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "product_variants.html")]
pub struct ProductVariantTemplate {
    pub variants: Vec<ProductVariantRow>,
    pub products: Vec<ProductResponseDto>, // <-- إضافة هذا الحقل
    pub brands: Vec<BrandResponseDto>,     // <-- إضافة هذا الحقل
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub edit_variant: Option<ProductVariantResponseDto>,
}

pub mod filters {
    use askama::Values;

    #[askama::filter_fn]
    pub fn first_letter(name: &str, _values: &dyn Values) -> askama::Result<String> {
        Ok(name
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string()))
    }

    #[askama::filter_fn]
    pub fn initial_color(name: &str, _values: &dyn Values) -> askama::Result<String> {
        const PALETTE: [&str; 6] =
            ["#0E7C66", "#2563EB", "#D97706", "#7C3AED", "#DB2777", "#0891B2"];
        let sum: u32 = name.bytes().map(|b| b as u32).sum();
        Ok(PALETTE[sum as usize % PALETTE.len()].to_string())
    }
}