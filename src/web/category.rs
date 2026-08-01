use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use crate::{
    domain::category::dto::{CategoryResponseDto, CategoryTemplate, CreateCategoryForm, UpdateCategoryForm}, state::AppState,
};

#[derive(Deserialize)]
pub struct EditQuery {
    pub edit: Option<i32>,
    pub error: Option<String>,
    pub success: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        // GET /categories -> List all categories or load edit mode
        .route("/", get(list_or_edit_categories_handler))
        // POST /categories -> Create a new category
        .route("/", post(create_category_handler))
        // POST /categories/{id}/update -> Update existing category via HTML form POST
        .route("/{id}/update", post(update_category_handler))
        // POST /categories/{id}/delete -> Delete category via HTML form POST
        .route("/{id}/delete", post(delete_category_handler))
}

// دالة عرض الصفحة الرئيسية (قائمة الفئات + نموذج الإنشاء أو التعديل)
pub async fn list_or_edit_categories_handler(
    State(state): State<AppState>,
    Query(query): Query<EditQuery>,
) -> impl IntoResponse {
    let pool = &state.pool;
    let categories = match sqlx::query_as::<_, CategoryResponseDto>(
        "SELECT id, name, name_ar, parent_id, notes, created_at, updated_at FROM categories ORDER BY id DESC"
    )
    .fetch_all(pool)
    .await {
        Ok(cats) => cats,
        Err(_) => vec![],
    };

    let edit_category = if let Some(edit_id) = query.edit {
        categories.iter().find(|c| c.id == edit_id).cloned()
    } else {
        None
    };

    CategoryTemplate {
        categories,
        error_message: query.error,
        success_message: query.success,
        edit_category,
    }
}

// دالة معالجة إنشاء فئة جديدة
pub async fn create_category_handler(
    State(state): State<AppState>,
    Form(form): Form<CreateCategoryForm>,
) -> impl IntoResponse {
     let pool = &state.pool;
    let existing_categories = match sqlx::query_as::<_, CategoryResponseDto>(
        "SELECT id, name, name_ar, parent_id, notes, created_at, updated_at FROM categories"
    )
    .fetch_all(pool)
    .await {
        Ok(cats) => cats,
        Err(_) => vec![],
    };

    if let Err(err_msg) = form.validate(&existing_categories) {
        let encoded_err = urlencoding::encode(&err_msg);
        return Redirect::to(&format!("/web/categories?error={}", encoded_err));
    }

    let result = sqlx::query(
        "INSERT INTO categories (name, name_ar, parent_id, notes, created_at, updated_at) VALUES ($1, $2, $3, $4, NOW(), NOW())"
    )
    .bind(&form.name)
    .bind(&form.name_ar)
    .bind(form.parent_id)
    .bind(&form.notes)
    .execute(pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/categories?success=تم إنشاء الفئة بنجاح"),
        Err(e) => {
           let error_string = format!("خطأ في قاعدة البيانات: {}", e);
let err_msg = urlencoding::encode(&error_string);
            Redirect::to(&format!("/web/categories?error={}", err_msg))
        }
    }
}

// دالة معالجة تحديث فئة موجودة
pub async fn update_category_handler(
     State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<UpdateCategoryForm>,
) -> impl IntoResponse {
     let pool = &state.pool;
    let existing_categories = match sqlx::query_as::<_, CategoryResponseDto>(
        "SELECT id, name, name_ar, parent_id, notes, created_at, updated_at FROM categories"
    )
    .fetch_all(pool)
    .await {
        Ok(cats) => cats,
        Err(_) => vec![],
    };

    if let Err(err_msg) = form.validate(id, &existing_categories) {
        let encoded_err = urlencoding::encode(&err_msg);
        return Redirect::to(&format!("/web/categories?edit={}&error={}", id, encoded_err));
    }

    let result = sqlx::query(
        "UPDATE categories SET name = $1, name_ar = $2, parent_id = $3, notes = $4, updated_at = NOW() WHERE id = $5"
    )
    .bind(&form.name)
    .bind(&form.name_ar)
    .bind(form.parent_id)
    .bind(&form.notes)
    .bind(id)
    .execute(pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/categories?success=تم تحديث الفئة بنجاح"),
        Err(e) => {
           let error_string = format!("فشل التحديث: {}", e);
let err_msg = urlencoding::encode(&error_string);
            Redirect::to(&format!("/web/categories?edit={}&error={}", id, err_msg))
        }
    }
}

// دالة حذف الفئة
pub async fn delete_category_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
     let pool = &state.pool;
    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(_) => Redirect::to("/web/categories?success=تم حذف الفئة بنجاح"),
        Err(e) => {
           let error_string = format!("لا يمكن حذف الفئة: {}", e);
let err_msg = urlencoding::encode(&error_string);
            Redirect::to(&format!("/web/categories?error={}", err_msg))
        }
    }
}