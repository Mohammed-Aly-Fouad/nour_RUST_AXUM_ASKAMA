use crate::domain::category::dto::{CategoryResponseDto, CreateCategoryApiDto, UpdateCategoryApiDto};
use crate::state::AppState;
use crate::common::ApiResponse;
use axum::{
    Json, Router, extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post, patch, delete},
};

/// Configures and returns the sub-router for all category-related JSON API endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        // GET /api/categories -> List all categories
        .route("/", get(list_categories))
        .route("/{id}", get(get_category))
        .route("/", post(create_category))
        // استخدام patch هو الأنسب للتحديث الجزئي للـ DTO الذي قمت ببنائه
        .route("/{id}", patch(update_category)) 
        .route("/{id}", delete(delete_category))
}



/// 1- Get All Categories
pub async fn list_categories(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let categories = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
        FROM categories
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await;

    match categories {
        Ok(cats) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Categories retrieved successfully".to_string(),
                data: Some(cats),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<Vec<CategoryResponseDto>> {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal server error occurred".to_string(),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}







/// 2. Retrieve a single brand by its unique ID via JSON API

pub async fn get_category(
    State(state): State<AppState>,
    Path(id_str): Path<String>, // 1. Extract as String first
) -> impl IntoResponse {
    // 2. Try to parse the string into an i32
    let id = match id_str.parse::<i32>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => {
            let response = ApiResponse::<CategoryResponseDto> {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: format!("Invalid ID format: '{}'. Expected a valid number.", id_str),
                data: None,
            };
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    // 3. Continue with your database query using the parsed `id`
    let category = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
        FROM categories
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await;

    match category {
        Ok(Some(b)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Category retrieved successfully".to_string(),
                data: Some(b),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response = ApiResponse::<CategoryResponseDto> {
                status: StatusCode::NOT_FOUND.as_u16(),
                message: format!("Category with ID {} not found", id),
                data: None,
            };
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<CategoryResponseDto> {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Internal server error occurred".to_string(),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}


/// 3- Create Category
pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryApiDto>,
) -> impl IntoResponse {
    // 1. Fetch existing categories to pass into your validation method
    let existing_categories = match sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
        FROM categories
        "#
    )
    .fetch_all(&state.pool)
    .await {
        Ok(cats) => cats,
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Internal server error occurred while fetching categories".to_string(),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response();
        }
    };

    // 2. Run your custom validation method
    if let Err(err_msg) = payload.validate(&existing_categories) {
        let response = ApiResponse::<()>::error(
            StatusCode::BAD_REQUEST.as_u16(),
            err_msg,
        );
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    // 3. If validation passes, insert the new category into the database
    let inserted_category = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        INSERT INTO categories (name, name_ar, parent_id, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, name_ar, parent_id, notes, created_at, updated_at
        "#,
        payload.name,
        payload.name_ar,
        payload.parent_id,
        payload.notes
    )
    .fetch_one(&state.pool)
    .await;

   match inserted_category {
        Ok(cat) => {
            let response = ApiResponse {
                status: StatusCode::CREATED.as_u16(),
                message: "Category created successfully".to_string(),
                data: Some(cat),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Category name already exists".to_string(), // رسالة واضحة للمستخدم
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to insert category into database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}



/// 4- Update Category

pub async fn update_category(
    State(state): State<AppState>,
    Path(category_id): Path<i32>,
    Json(payload): Json<UpdateCategoryApiDto>,
) -> impl IntoResponse {
    // 1. جلب الفئة القديمة من قاعدة البيانات للتأكد من وجودها
    let old_category = match sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
        FROM categories
        WHERE id = $1
        "#,
        category_id
    )
    .fetch_optional(&state.pool)
    .await {
        Ok(Some(cat)) => cat,
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                "Category not found".to_string(),
            );
            return (StatusCode::NOT_FOUND, Json(response)).into_response();
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Database error while fetching category".to_string(),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response();
        }
    };

    // 2. دمج البيانات الجديدة مع القديمة (لأن الحقول في PATCH اختيارية Option)
    // إذا أرسل المستخدم قيمة جديدة نعتمدها، وإذا أرسل None نترك القيمة القديمة كما هي
    let final_name = payload.name.as_deref().unwrap_or(&old_category.name);
    let final_name_ar = payload.name_ar.as_deref().unwrap_or(&old_category.name_ar);
    
    // بالنسبة للـ parent_id، التعامل مع الـ Option يحتاج دقة أكبر
    // إذا أرسل المستخدم Some(val) نأخذها، وإذا أرسل None نتحقق هل يقصد إبقائها كما هي أم إلغاءها؟
    // للاختصار والبساطة المباشرة، سنعتبر أن الـ payload يحدد القيمة الجديدة أو نتركها للقديمة:
    let final_parent_id = match payload.parent_id {
        Some(val) => Some(val),
        None => old_category.parent_id,
    };

    let final_notes = match &payload.notes {
        Some(val) => Some(val.clone()),
        None => old_category.notes.clone(),
    };

    // 3. التحقق مما إذا كانت البيانات مطابقة تماماً للقديمة (لا يوجد أي تغيير)
    if final_name == old_category.name
        && final_name_ar == old_category.name_ar
        && final_parent_id == old_category.parent_id
        && final_notes == old_category.notes
    {
        // البيانات مطابقة تماماً، نعيد البيانات القديمة مباشرة دون تنفيذ UPDATE
        let response = ApiResponse {
            status: StatusCode::OK.as_u16(),
            message: "No changes detected, category remains unchanged".to_string(),
            data: Some(old_category),
        };
        return (StatusCode::OK, Json(response)).into_response();
    }

    // 4. جلب جميع الفئات لتطبيق نفس شروط الـ validate (مثل منع تكرار الأسماء وفحص الأب)
    let existing_categories = match sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
        FROM categories
        WHERE id != $1 -- استثناء الفئة الحالية حتى لا تقارن نفسها بنفسها في التكرار
        "#,
        category_id
    )
    .fetch_all(&state.pool)
    .await {
        Ok(cats) => cats,
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Internal server error occurred".to_string(),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response();
        }
    };

    // 5. إجراء الفحص (Validation) على البيانات المدمجة الجديدة
    // (يمكنك بناء دالة validate مشابهة تستقبل القيم النهائية للتحقق منها)
    let trimmed_name = final_name.trim();
    if trimmed_name.is_empty() {
        let response = ApiResponse::<()>::error(StatusCode::BAD_REQUEST.as_u16(), "English name is required".to_string());
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }
    
    let trimmed_name_ar = final_name_ar.trim();
    if trimmed_name_ar.is_empty() {
        let response = ApiResponse::<()>::error(StatusCode::BAD_REQUEST.as_u16(), "Arabic name is required".to_string());
        return (StatusCode::BAD_REQUEST, Json(response)).into_response();
    }

    // التحقق من أن الأب موجود وأنه فئة أساسية وليست فرعية
    if let Some(pid) = final_parent_id {
        if pid == category_id {
            let response = ApiResponse::<()>::error(StatusCode::BAD_REQUEST.as_u16(), "Category cannot be parent of itself".to_string());
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
        let parent_exists = existing_categories.iter().any(|cat| cat.id == pid && cat.parent_id.is_none());
        if !parent_exists {
            let response = ApiResponse::<()>::error(StatusCode::BAD_REQUEST.as_u16(), "Invalid parent category".to_string());
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    }

    // 6. تنفيذ عملية التحديث (UPDATE) في قاعدة البيانات
    let updated_category = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        UPDATE categories
        SET name = $1, name_ar = $2, parent_id = $3, notes = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING id, name, name_ar, parent_id, notes, created_at, updated_at
        "#,
        final_name,
        final_name_ar,
        final_parent_id,
        final_notes,
        category_id
    )
    .fetch_one(&state.pool)
    .await;

    match updated_category {
        Ok(cat) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Category updated successfully".to_string(),
                data: Some(cat),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Category name already exists".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to update category in database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}


/// 5- Delete Category


pub async fn delete_category(
    State(state): State<AppState>,
    Path(category_id): Path<i32>,
) -> impl IntoResponse {
    // 1. محاولة حذف الفئة مباشرة من قاعدة البيانات والتأكد مما إذا كانت موجودة أم لا
    let result = sqlx::query!(
        r#"
        DELETE FROM categories
        WHERE id = $1
        RETURNING id
        "#,
        category_id
    )
    .fetch_optional(&state.pool)
    .await;

    match result {
        Ok(Some(_)) => {
            // الحالة الأولى: تم العثور على الفئة وحذفها بنجاح
            let response = ApiResponse::<()>::error( // أو يمكنك استخدام هيكل مخصص للنجاح بدون بيانات
                StatusCode::OK.as_u16(),
                "Category deleted successfully".to_string(),
            );
            // ملاحظة: لو كنت تريد استخدام هيكل ناجح يحتوي على data: None يمكنك تخصيصه هكذا:
            let success_response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Category deleted successfully".to_string(),
                data: None::<()>,
            };
            (StatusCode::OK, Json(success_response)).into_response()
        }
        Ok(None) => {
            // الحالة الثانية: المعرّف غير موجود في قاعدة البيانات
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                "Category not found".to_string(),
            );
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(_) => {
            // حالة حدوث خطأ تقني في قاعدة البيانات
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to delete category from database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}