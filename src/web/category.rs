use axum::{
    Router, extract::{Form, Path, Query, State}, http::StatusCode, response::{IntoResponse, Redirect}, routing::{get, post},
};
use serde::Deserialize;

use crate::domain::category::dto::{
    CategoryResponseDto, CategoryRow, CategorySearchQuery, CategorySearchResultsTemplate, CategoryTemplate, CreateCategoryForm, UpdateCategoryForm,
};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_categories_page).post(create_category_web))
        .route("/edit/{id}", get(edit_category_page))
        .route("/update/{id}", post(update_category_web))
        .route("/delete/{id}", post(delete_category_web))
}

#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub ok: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

async fn fetch_all_categories(state: &AppState) -> Vec<CategoryResponseDto> {
    sqlx::query_as::<_, CategoryResponseDto>(
        "SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
         FROM categories ORDER BY id DESC",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

async fn fetch_category_by_id(state: &AppState, id: i32) -> Option<CategoryResponseDto> {
    sqlx::query_as::<_, CategoryResponseDto>(
        "SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
         FROM categories WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(None)
}

/// الفئات الجذعية بس (لملء قائمة اختيار "الفئة الرئيسية")
/// exclude_id: نستبعد بيه الفئة الحالية وقت التعديل (فئة ميقدرش تبقى أب لنفسها)
fn build_root_categories(
    categories: &[CategoryResponseDto],
    exclude_id: Option<i64>,
) -> Vec<CategoryResponseDto> {
    categories
        .iter()
        .filter(|c| c.parent_id.is_none() && Some(c.id) != exclude_id)
        .cloned()
        .collect()
}

fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505"))
}

fn is_foreign_key_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503"))
}

// ==================== 1. Render main page ====================

pub async fn render_categories_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> CategoryTemplate {
    let success_message = match params.ok.as_deref() {
        Some("created") => Some("تم إنشاء الفئة بنجاح".to_string()),
        Some("updated") => Some("تم تحديث الفئة بنجاح".to_string()),
        Some("deleted") => Some("تم حذف الفئة بنجاح".to_string()),
        _ => None,
    };

    let all = fetch_all_categories(&state).await;
    CategoryTemplate {
        root_categories: build_root_categories(&all, None),
        categories: CategoryRow::build_rows(&all),
        error_message: None,
        success_message,
        current_page: "categories".to_string(),
        edit_category: None,
    }
}

// ==================== 2. Create Category ====================

pub async fn create_category_web(
    State(state): State<AppState>,
    Form(form): Form<CreateCategoryForm>,
) -> axum::response::Response {
    let all = fetch_all_categories(&state).await;

    if let Err(err_msg) = form.validate(&all) {
        return CategoryTemplate {
            root_categories: build_root_categories(&all, None),
            categories: CategoryRow::build_rows(&all),
            error_message: Some(err_msg),
            success_message: None,
            current_page: "categories".to_string(),
            edit_category: None,
        }
        .into_response();
    }

    let result = sqlx::query(
        "INSERT INTO categories (name, name_ar, parent_id, notes, created_at, updated_at)
         VALUES ($1, $2, $3, $4, NOW(), NOW())",
    )
    .bind(form.name.trim())
    .bind(form.name_ar.trim())
    .bind(form.parent_id)
    .bind(&form.notes)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/categories?ok=created").into_response(),
        Err(e) => {
            let all = fetch_all_categories(&state).await;
            let msg = if is_unique_violation(&e) {
                "اسم الفئة موجود بالفعل".to_string()
            } else {
                "حدث خطأ أثناء إضافة الفئة، حاول مرة أخرى".to_string()
            };
            CategoryTemplate {
                root_categories: build_root_categories(&all, None),
                categories: CategoryRow::build_rows(&all),
                error_message: Some(msg),
                success_message: None,
                edit_category: None,
                current_page: "categories".to_string(),
            }
            .into_response()
        }
    }
}

// ==================== 3. Edit page (GET) ====================

pub async fn edit_category_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> CategoryTemplate {
    let all = fetch_all_categories(&state).await;
    let edit_category = all.iter().find(|c| c.id == id).cloned();

    CategoryTemplate {
        root_categories: build_root_categories(&all, Some(id)),
        categories: CategoryRow::build_rows(&all),
        error_message: None,
        success_message: None,
        edit_category,
        current_page: "categories".to_string(),
    }
}

// ==================== 4. Update Category ====================

pub async fn update_category_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateCategoryForm>,
) -> axum::response::Response {
    let all = fetch_all_categories(&state).await;

    let old_category = match all.iter().find(|c| c.id == id).cloned() {
        Some(c) => c,
        None => {
            return CategoryTemplate {
                root_categories: build_root_categories(&all, Some(id)),
                categories: CategoryRow::build_rows(&all),
                error_message: Some("الفئة غير موجودة".to_string()),
                success_message: None,
                edit_category: None,
                current_page: "categories".to_string(),
            }
            .into_response();
        }
    };

    if let Err(err_msg) = form.validate(id, &all) {
        return CategoryTemplate {
            root_categories: build_root_categories(&all, Some(id)),
            categories: CategoryRow::build_rows(&all),
            error_message: Some(err_msg),
            success_message: None,
            edit_category: Some(old_category),
            current_page: "categories".to_string(),
        }
        .into_response();
    }

    let result = sqlx::query(
        "UPDATE categories
         SET name = $1, name_ar = $2, parent_id = $3, notes = $4, updated_at = NOW()
         WHERE id = $5",
    )
    .bind(form.name.trim())
    .bind(form.name_ar.trim())
    .bind(form.parent_id)
    .bind(&form.notes)
    .bind(id)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/categories?ok=updated").into_response(),
        Err(e) => {
            let all = fetch_all_categories(&state).await;
            let msg = if is_unique_violation(&e) {
                "اسم الفئة موجود بالفعل".to_string()
            } else {
                "حدث خطأ أثناء تحديث الفئة، حاول مرة أخرى".to_string()
            };
            CategoryTemplate {
                root_categories: build_root_categories(&all, Some(id)),
                categories: CategoryRow::build_rows(&all),
                error_message: Some(msg),
                success_message: None,
                edit_category: Some(old_category),
                current_page: "categories".to_string(),
            }
            .into_response()
        }
    }
}

// ==================== 5. Delete Category ====================

pub async fn delete_category_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Redirect::to("/web/categories?ok=deleted").into_response(),
        Err(e) => {
            let all = fetch_all_categories(&state).await;
            let msg = if is_foreign_key_violation(&e) {
                "لا يمكن حذف هذه الفئة لأنها مرتبطة بفئات فرعية أو منتجات. قم بإزالتها أو نقلها أولاً.".to_string()
            } else {
                "حدث خطأ أثناء حذف الفئة".to_string()
            };
            CategoryTemplate {
                root_categories: build_root_categories(&all, None),
                categories: CategoryRow::build_rows(&all),
                error_message: Some(msg),
                success_message: None,
                edit_category: None,
                current_page: "categories".to_string(),
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
pub async fn search_categories_handler(
    State(state): State<AppState>,
    Query(query): Query<CategorySearchQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let q = query.q.trim();

    // إرجاع استجابة فارغة فوراً إن كان الاستعلام خالياً
    if q.is_empty() {
        return Ok(CategorySearchResultsTemplate {
            categories: vec![],
            query: String::new(),
        });
    }

    // إعداد نمط البحث غير حساس للحالة (Case-Insensitive) للغتين العربية والإنجليزية
    let search_pattern = format!("%{}%", q);

    let categories = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
        FROM categories
        WHERE name ILIKE $1 OR name_ar ILIKE $1
        ORDER BY name_ar ASC
        LIMIT 10
        "#,
        search_pattern
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute category search query: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(CategorySearchResultsTemplate {
        categories,
        query: q.to_string(),
    })
}