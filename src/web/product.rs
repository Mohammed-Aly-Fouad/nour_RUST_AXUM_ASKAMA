use axum::{
    Router, extract::{Form, Path, Query, State}, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post},
};
use serde::Deserialize;

use crate::domain::{category::dto::CategoryResponseDto, product::dto::{ProductSearchQuery, ProductsearchResultsTemplate}};
use crate::domain::product::dto::{
    CreateProductForm, ProductResponseDto, ProductsTemplate, UpdateProductForm,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_products_page).post(create_product_web))
        .route("/edit/{id}", get(edit_product_page))
        .route("/update/{id}", post(update_product_web))
        .route("/delete/{id}", post(delete_product_web))
        .route("/search", get(search_products_handler))
}

#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub ok: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn fetch_all_products(state: &AppState) -> Vec<ProductResponseDto> {
    sqlx::query_as::<_, ProductResponseDto>(
        "SELECT id, name, name_ar, category_id, notes, created_at, updated_at
         FROM products ORDER BY id DESC",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

async fn fetch_all_categories(state: &AppState) -> Vec<CategoryResponseDto> {
    sqlx::query_as::<_, CategoryResponseDto>(
        "SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
FROM categories 
WHERE parent_id IS NOT NULL 
ORDER BY name_ar ASC;",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505"))
}

fn is_foreign_key_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503"))
}

// ==================== 1. Render main page ====================

pub async fn render_products_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> ProductsTemplate {
    let success_message = match params.ok.as_deref() {
        Some("created") => Some("تم إنشاء المنتج بنجاح".to_string()),
        Some("updated") => Some("تم تحديث المنتج بنجاح".to_string()),
        Some("deleted") => Some("تم حذف المنتج بنجاح".to_string()),
        _ => None,
    };

    let products = fetch_all_products(&state).await;
    let categories = fetch_all_categories(&state).await;

    ProductsTemplate {
        categories,
        products,
        error_message: None,
        success_message,
        edit_product: None,
        current_page: "products".to_string(),
        
    }
}

// ==================== 2. Create Product ====================

pub async fn create_product_web(
    State(state): State<AppState>,
    Form(form): Form<CreateProductForm>,
) -> axum::response::Response {
    let products = fetch_all_products(&state).await;
    let categories = fetch_all_categories(&state).await;

    // 1. Structural validation
    if let Err(err_msg) = form.validate() {
        return ProductsTemplate {
            categories,
            products,
            error_message: Some(err_msg),
            success_message: None,
            edit_product: None,
            current_page: "products".to_string(),
        }
        .into_response();
    }

    // 2. Business Logic / Database duplication check in Handler
    let name_trimmed = form.name.trim();
    if products.iter().any(|p| p.name.eq_ignore_ascii_case(name_trimmed)) {
        return ProductsTemplate {
            categories,
            products,
            error_message: Some("اسم المنتج بالإنجليزية موجود بالفعل".to_string()),
            success_message: None,
            edit_product: None,
            current_page: "products".to_string(),
        }
        .into_response();
    }

    // 3. Insert into Database
    let result = sqlx::query(
        "INSERT INTO products (name, name_ar, category_id, notes, created_at, updated_at)
         VALUES ($1, $2, $3, $4, NOW(), NOW())",
    )
    .bind(name_trimmed)
    .bind(form.name_ar.trim())
    .bind(form.category_id)
    .bind(&form.notes)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/products?ok=created").into_response(),
        Err(e) => {
            let products = fetch_all_products(&state).await;
            let categories = fetch_all_categories(&state).await;
            let msg = if is_unique_violation(&e) {
                "اسم المنتج موجود بالفعل".to_string()
            } else {
                "حدث خطأ أثناء إضافة المنتج، حاول مرة أخرى".to_string()
            };
            ProductsTemplate {
                categories,
                products,
                error_message: Some(msg),
                success_message: None,
                edit_product: None,
                current_page: "products".to_string(),
            }
            .into_response()
        }
    }
}

// ==================== 3. Edit page (GET) ====================

pub async fn edit_product_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ProductsTemplate {
    let products = fetch_all_products(&state).await;
    let categories = fetch_all_categories(&state).await;
    let edit_product = products.iter().find(|p| p.id == id).cloned();

    ProductsTemplate {
        categories,
        products,
        error_message: None,
        success_message: None,
        edit_product,
        current_page: "products".to_string(),
    }
}

// ==================== 4. Update Product ====================

pub async fn update_product_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateProductForm>,
) -> axum::response::Response {
    let products = fetch_all_products(&state).await;
    let categories = fetch_all_categories(&state).await;

    let old_product = match products.iter().find(|p| p.id == id).cloned() {
        Some(p) => p,
        None => {
            return ProductsTemplate {
                categories,
                products,
                error_message: Some("المنتج غير موجود".to_string()),
                success_message: None,
                edit_product: None,
                current_page: "products".to_string(),
            }
            .into_response();
        }
    };

    // 1. Structural validation
    if let Err(err_msg) = form.validate() {
        return ProductsTemplate {
            categories,
            products,
            error_message: Some(err_msg),
            success_message: None,
            edit_product: Some(old_product),
            current_page: "products".to_string(),
        }
        .into_response();
    }

    // 2. Business Logic / Duplication check (Excluding current item ID)
    let name_trimmed = form.name.trim();
    if products
        .iter()
        .any(|p| p.id != id && p.name.eq_ignore_ascii_case(name_trimmed))
    {
        return ProductsTemplate {
            categories,
            products,
            error_message: Some("اسم المنتج بالإنجليزية مستخدم بالفعل لمنتج آخر".to_string()),
            success_message: None,
            edit_product: Some(old_product),
            current_page: "products".to_string(),
        }
        .into_response();
    }

    // 3. Update Database
    let result = sqlx::query(
        "UPDATE products
         SET name = $1, name_ar = $2, category_id = $3, notes = $4, updated_at = NOW()
         WHERE id = $5",
    )
    .bind(name_trimmed)
    .bind(form.name_ar.trim())
    .bind(form.category_id)
    .bind(&form.notes)
    .bind(id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/products?ok=updated").into_response(),
        Err(e) => {
            let products = fetch_all_products(&state).await;
            let categories = fetch_all_categories(&state).await;
            let msg = if is_unique_violation(&e) {
                "اسم المنتج موجود بالفعل".to_string()
            } else {
                "حدث خطأ أثناء تحديث المنتج، حاول مرة أخرى".to_string()
            };
            ProductsTemplate {
                categories,
                products,
                error_message: Some(msg),
                success_message: None,
                edit_product: Some(old_product),
                current_page: "products".to_string(),
            }
            .into_response()
        }
    }
}

// ==================== 5. Delete Product ====================

pub async fn delete_product_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Redirect::to("/web/products?ok=deleted").into_response(),
        Err(e) => {
            let products = fetch_all_products(&state).await;
            let categories = fetch_all_categories(&state).await;
            let msg = if is_foreign_key_violation(&e) {
                "لا يمكن حذف هذا المنتج لأنه مرتبط ببيانات أخرى (مثل المتغيرات). قم بإزالتها أولاً.".to_string()
            } else {
                "حدث خطأ أثناء حذف المنتج".to_string()
            };
            ProductsTemplate {
                categories,
                products,
                error_message: Some(msg),
                success_message: None,
                edit_product: None,
                current_page: "products".to_string(),
            }
            .into_response()
        }
    }
}


// ============================================================================
// HANDLERS: LIVE SEARCH
// ============================================================================

/// Dynamic search handler returning a rendered Askama partial snippet.
/// Designed for live search / auto-complete integrations.
pub async fn search_products_handler(
    State(state): State<AppState>,
    Query(query): Query<ProductSearchQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let q = query.q.trim();

    // إرجاع استجابة فارغة فوراً إن كان الاستعلام خالياً
    if q.is_empty() {
        return Ok(ProductsearchResultsTemplate {
            products: vec![],
            query: String::new(),
        });
    }

    // إعداد نمط البحث غير حساس للحالة (Case-Insensitive) للغتين العربية والإنجليزية
    let search_pattern = format!("%{}%", q);

    let products = sqlx::query_as!(
        ProductResponseDto,
        r#"
        SELECT id, name, name_ar ,category_id, notes, created_at, updated_at
        FROM products
        WHERE name ILIKE $1 OR name_ar ILIKE $1
        ORDER BY name_ar ASC
        LIMIT 10
        "#,
        search_pattern
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute brand search query: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(ProductsearchResultsTemplate {
        products,
        query: q.to_string(),
    })
}