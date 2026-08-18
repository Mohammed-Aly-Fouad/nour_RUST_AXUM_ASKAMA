use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::domain::brand::dto::BrandResponseDto;
use crate::domain::product::dto::ProductResponseDto;
use crate::domain::product_variant::dto::{
    CreateProductVariantForm, ProductVariantResponseDto, ProductVariantRow,
    ProductVariantTemplate, UpdateProductVariantForm,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_variants_page).post(create_variant_web))
        .route("/edit/{id}", get(edit_variant_page))
        .route("/update/{id}", post(update_variant_web))
        .route("/delete/{id}", post(delete_variant_web))
}

#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub ok: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn fetch_all_variants(state: &AppState) -> Vec<ProductVariantResponseDto> {
    sqlx::query_as::<_, ProductVariantResponseDto>(
        r#"
        SELECT id, product_id, brand_id, name, name_ar, sku, barcode,
               shelf_location, stock_quantity, reorder_threshold, is_active,
               attr, notes, created_at, updated_at
        FROM product_variants
        ORDER BY id DESC
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

async fn fetch_all_products(state: &AppState) -> Vec<ProductResponseDto> {
    sqlx::query_as!(
        ProductResponseDto,
        r#"SELECT id, category_id, name, name_ar, notes, created_at, updated_at FROM products ORDER BY name_ar ASC"#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

async fn fetch_all_brands(state: &AppState) -> Vec<BrandResponseDto> {
    sqlx::query_as!(
        BrandResponseDto,
        r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands ORDER BY name_ar ASC"#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

async fn fetch_products_map(state: &AppState) -> HashMap<i64, String> {
    sqlx::query!(
        r#"SELECT id, name_ar FROM products"#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|r| (r.id, r.name_ar))
    .collect()
}

async fn fetch_brands_map(state: &AppState) -> HashMap<i64, String> {
    sqlx::query!(
        r#"SELECT id, name_ar FROM brands"#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|r| (r.id, r.name_ar))
    .collect()
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505"))
}

fn is_foreign_key_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503"))
}

async fn build_template(
    state: &AppState,
    error_message: Option<String>,
    success_message: Option<String>,
    edit_variant: Option<ProductVariantResponseDto>,
) -> ProductVariantTemplate {
    let variants = fetch_all_variants(state).await;
    let products_map = fetch_products_map(state).await;
    let brands_map = fetch_brands_map(state).await;

    let products = fetch_all_products(state).await;
    let brands = fetch_all_brands(state).await;

    let rows = ProductVariantRow::build_rows(&variants, &products_map, &brands_map);

    ProductVariantTemplate {
        variants: rows,
        products,
        brands,
        error_message,
        success_message,
        edit_variant,
        current_page: "variants".to_string(),
}
    }


// ==================== 1. Render main page ====================

pub async fn render_variants_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> ProductVariantTemplate {
    let success_message = match params.ok.as_deref() {
        Some("created") => Some("تم إنشاء متغير المنتج بنجاح".to_string()),
        Some("updated") => Some("تم تحديث متغير المنتج بنجاح".to_string()),
        Some("deleted") => Some("تم حذف متغير المنتج بنجاح".to_string()),
        _ => None,
    };

    build_template(&state, None, success_message, None).await
}

// ==================== 2. Create Product Variant ====================

pub async fn create_variant_web(
    State(state): State<AppState>,
    Form(form): Form<CreateProductVariantForm>,
) -> axum::response::Response {
    let existing_variants = fetch_all_variants(&state).await;

    // 1. Validation (In-Memory checks)
    if let Err(err_msg) = form.validate(&existing_variants) {
        return build_template(&state, Some(err_msg), None, None)
            .await
            .into_response();
    }

    // 2. Database Insert
    let stock_qty = form.stock_quantity.unwrap_or(0);
    let reorder_thresh = form.reorder_threshold.unwrap_or(0);
    let active = form.is_active.unwrap_or(true);

    let result = sqlx::query(
        r#"
        INSERT INTO product_variants (
            product_id, brand_id, name, name_ar, sku, barcode,
            shelf_location, stock_quantity, reorder_threshold, is_active,
            attr, notes, created_at, updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), NOW())
        "#,
    )
    .bind(form.product_id)
    .bind(form.brand_id)
    .bind(form.name.trim())
    .bind(form.name_ar.trim())
    .bind(form.sku.trim())
    .bind(form.barcode.as_ref().map(|s| s.trim()))
    .bind(form.shelf_location.as_ref().map(|s| s.trim()))
    .bind(stock_qty)
    .bind(reorder_thresh)
    .bind(active)
    .bind(serde_json::json!({}))
    .bind(&form.notes)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/product-variants?ok=created").into_response(),
        Err(e) => {
            let msg = if is_unique_violation(&e) {
                "رمز SKU أو الباركود مستخدم بالفعل".to_string()
            } else if is_foreign_key_violation(&e) {
                "المنتج أو الماركة المحددة غير موجودة".to_string()
            } else {
                "حدث خطأ أثناء إضافة متغير المنتج، حاول مرة أخرى".to_string()
            };

            build_template(&state, Some(msg), None, None)
                .await
                .into_response()
        }
    }
}

// ==================== 3. Edit page (GET) ====================

pub async fn edit_variant_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ProductVariantTemplate {
    let variants = fetch_all_variants(&state).await;
    let edit_variant = variants.iter().find(|v| v.id == id).cloned();

    build_template(&state, None, None, edit_variant).await
}

// ==================== 4. Update Product Variant ====================

pub async fn update_variant_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateProductVariantForm>,
) -> axum::response::Response {
    let existing_variants = fetch_all_variants(&state).await;

    let old_variant = match existing_variants.iter().find(|v| v.id == id).cloned() {
        Some(v) => v,
        None => {
            return build_template(&state, Some("متغير المنتج غير موجود".to_string()), None, None)
                .await
                .into_response();
        }
    };

    // 1. Validation (In-Memory checks excluding current item ID)
    if let Err(err_msg) = form.validate(id, &existing_variants) {
        return build_template(&state, Some(err_msg), None, Some(old_variant))
            .await
            .into_response();
    }

    // 2. Update Database
    let stock_qty = form.stock_quantity.unwrap_or(0);
    let reorder_thresh = form.reorder_threshold.unwrap_or(0);
    let active = form.is_active.unwrap_or(true);

    let result = sqlx::query(
        r#"
        UPDATE product_variants
        SET product_id = $1,
            brand_id = $2,
            name = $3,
            name_ar = $4,
            sku = $5,
            barcode = $6,
            shelf_location = $7,
            stock_quantity = $8,
            reorder_threshold = $9,
            is_active = $10,
            notes = $11,
            updated_at = NOW()
        WHERE id = $12
        "#,
    )
    .bind(form.product_id)
    .bind(form.brand_id)
    .bind(form.name.trim())
    .bind(form.name_ar.trim())
    .bind(form.sku.trim())
    .bind(form.barcode.as_ref().map(|s| s.trim()))
    .bind(form.shelf_location.as_ref().map(|s| s.trim()))
    .bind(stock_qty)
    .bind(reorder_thresh)
    .bind(active)
    .bind(&form.notes)
    .bind(id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/product-variants?ok=updated").into_response(),
        Err(e) => {
            let msg = if is_unique_violation(&e) {
                "رمز SKU أو الباركود مستخدم بالفعل لمتغير آخر".to_string()
            } else if is_foreign_key_violation(&e) {
                "المنتج أو الماركة المحددة غير موجودة".to_string()
            } else {
                "حدث خطأ أثناء تحديث متغير المنتج، حاول مرة أخرى".to_string()
            };

            build_template(&state, Some(msg), None, Some(old_variant))
                .await
                .into_response()
        }
    }
}

// ==================== 5. Delete Product Variant ====================

pub async fn delete_variant_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    let result = sqlx::query(
        r#"DELETE FROM product_variants WHERE id = $1"#,
    )
    .bind(id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/product-variants?ok=deleted").into_response(),
        Err(e) => {
            let msg = if is_foreign_key_violation(&e) {
                "لا يمكن حذف هذا المتغير لأنه مرتبط بسجلات أخرى (مثل حركة المخزون أو الفواتير)".to_string()
            } else {
                "حدث خطأ أثناء حذف متغير المنتج، حاول مرة أخرى".to_string()
            };

            build_template(&state, Some(msg), None, None)
                .await
                .into_response()
        }
    }
}