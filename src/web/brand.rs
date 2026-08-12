use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::domain::brand::dto::{
    BrandSearchQuery, BrandSearchResultsTemplate, BrandResponseDto, BrandsTemplate,
    CreateBrandForm, MergedBrandFormData, UpdateBrandForm,
};
use crate::state::AppState;

// ============================================================================
// ROUTER CONFIGURATION
// ============================================================================

/// Configures and returns the sub-router for all browser-based Askama HTML endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        // GET /brands -> Render main page | POST /brands -> Handle creation
        .route("/", get(render_brands_page).post(create_brand_web))
        // GET /brands/edit/{id} -> Render edit modal/view pre-loaded with data
        .route("/edit/{id}", get(edit_brand_page))
        // POST /brands/update/{id} -> Submit brand update
        .route("/update/{id}", post(update_brand_web))
        // POST /brands/delete/{id} -> Handle brand deletion
        .route("/delete/{id}", post(delete_brand_web))
        // GET /brands/search -> Live search endpoint for HTMX / Fetch API
        .route("/search", get(search_brands_handler))
}

// ============================================================================
// FLASH MESSAGES & QUERY PARAMS
// ============================================================================

/// Query parameters used to carry one-time "Flash Messages" across HTTP Redirects.
///
/// **لماذا نستخدم هذا النمط؟**
/// بعد أي عملية نجاح (إنشاء / تعديل / حذف)، نقوم بعمل `Redirect` لمنع تكرار الإرسال
/// عند عمل (Refresh). وبما أن الـ Redirect ينشئ طلب `GET` جديد تماماً، تفقد الاستجابة
/// بيانات الـ Context السابقة. نمرر إشارة مثل (`?ok=created`) ونقرأها في صفحة العرض.
#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub ok: Option<String>,
}

// ============================================================================
// DATABASE HELPER FUNCTIONS
// ============================================================================

/// يجلب كافة البراندات مرتبة تنازلياً حسب المعرف (`ID`).
/// 
/// في حالة حدوث خطأ في قاعدة البيانات، تُرجع الدالة قائمة فارغة لضمان
/// استمرارية عمل واجهة المستخدم وعدم توقف الصفحة بالكامل.
async fn fetch_all_brands(state: &AppState) -> Vec<BrandResponseDto> {
    sqlx::query_as!(
        BrandResponseDto,
        r#"
        SELECT id, name, name_ar, notes, created_at, updated_at 
        FROM brands 
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

/// يجلب براند واحد عن طريق الـ `ID` لتحميل بياناته مسبقاً في نموذج التعديل.
async fn fetch_brand_by_id(state: &AppState, id: i64) -> Option<BrandResponseDto> {
    sqlx::query_as!(
        BrandResponseDto,
        r#"
        SELECT id, name, name_ar, notes, created_at, updated_at 
        FROM brands 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(None)
}

// ============================================================================
// HANDLERS: READ & RENDER
// ============================================================================

/// Renders the main HTML page containing the brand list and creation form.
///
/// Checks for flash params (`?ok=...`) in the query string to display contextual 
/// success notifications after a redirect.
pub async fn render_brands_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> BrandsTemplate {
    let success_message = match params.ok.as_deref() {
        Some("created") => Some("تم إضافة البراند بنجاح".to_string()),
        Some("updated") => Some("تم تعديل البراند بنجاح".to_string()),
        Some("deleted") => Some("تم حذف البراند بنجاح".to_string()),
        _ => None,
    };

    BrandsTemplate {
        brands: fetch_all_brands(&state).await,
        error_message: None,
        success_message,
        edit_brand: None,
    }
}

/// Renders the main page with a specific brand pre-loaded for editing inside a modal or inline form.
pub async fn edit_brand_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> BrandsTemplate {
    BrandsTemplate {
        brands: fetch_all_brands(&state).await,
        error_message: None,
        success_message: None,
        edit_brand: fetch_brand_by_id(&state, id).await,
    }
}

// ============================================================================
// HANDLERS: MUTATION (CREATE, UPDATE, DELETE)
// ============================================================================

/// Handles web form submission for creating a new brand.
///
/// Validates input data and handles PostgreSQL unique constraint violations (`23505`).
pub async fn create_brand_web(
    State(state): State<AppState>,
    Form(form): Form<CreateBrandForm>,
) -> Response {
    // 1. التحقق من صحة القواعد الإدخالية (Validation)
    if let Err(err_msg) = form.validate() {
        return BrandsTemplate {
            brands: fetch_all_brands(&state).await,
            error_message: Some(err_msg),
            success_message: None,
            edit_brand: None,
        }
        .into_response();
    }

    let trimmed_name = form.name.trim();
    let trimmed_name_ar = form.name_ar.trim();

    // 2. إدخال البيانات في قاعدة البيانات
    let result = sqlx::query!(
        r#"INSERT INTO brands (name, name_ar, notes) VALUES ($1, $2, $3)"#,
        trimmed_name,
        trimmed_name_ar,
        form.notes
    )
    .execute(&state.pool)
    .await;

    // 3. معالجة النتيجة والتوجيه
    match result {
        Ok(_) => Redirect::to("/web/brands?ok=created").into_response(),
        // Error 23505: Unique Violation (اسم البراند مكرر)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            BrandsTemplate {
                brands: fetch_all_brands(&state).await,
                error_message: Some("اسم البراند موجود بالفعل".to_string()),
                success_message: None,
                edit_brand: None,
            }
            .into_response()
        }
        Err(_) => BrandsTemplate {
            brands: fetch_all_brands(&state).await,
            error_message: Some("حدث خطأ أثناء إضافة البراند، حاول مرة أخرى".to_string()),
            success_message: None,
            edit_brand: None,
        }
        .into_response(),
    }
}

/// Handles web form submission for updating an existing brand.
///
/// Uses `MergedBrandFormData` to enforce consistent validation logic across Web and API routes.
pub async fn update_brand_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateBrandForm>,
) -> Response {
    // 1. التحقق من وجود البراند المراد تعديله
    let old_brand = match fetch_brand_by_id(&state, id).await {
        Some(b) => b,
        None => {
            return BrandsTemplate {
                brands: fetch_all_brands(&state).await,
                error_message: Some("البراند غير موجود".to_string()),
                success_message: None,
                edit_brand: None,
            }
            .into_response();
        }
    };

    // 2. التحقق من صحة المدخلات بدمج الهيكل مع قواعد التحقق الموحدة
    let merged = MergedBrandFormData {
        name: &form.name,
        name_ar: &form.name_ar,
    };

    if let Err(err_msg) = merged.validate() {
        return BrandsTemplate {
            brands: fetch_all_brands(&state).await,
            error_message: Some(err_msg),
            success_message: None,
            edit_brand: Some(old_brand),
        }
        .into_response();
    }

    let trimmed_name = form.name.trim();
    let trimmed_name_ar = form.name_ar.trim();

    // 3. تنفيذ التحديث
    let result = sqlx::query!(
        r#"
        UPDATE brands 
        SET name = $1, name_ar = $2, notes = $3, updated_at = now() 
        WHERE id = $4
        "#,
        trimmed_name,
        trimmed_name_ar,
        form.notes,
        id
    )
    .execute(&state.pool)
    .await;

    // 4. معالجة النتيجة والتوجيه
    match result {
        Ok(_) => Redirect::to("/web/brands?ok=updated").into_response(),
        // Error 23505: Unique Violation
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            BrandsTemplate {
                brands: fetch_all_brands(&state).await,
                error_message: Some("اسم البراند موجود بالفعل".to_string()),
                success_message: None,
                edit_brand: Some(old_brand),
            }
            .into_response()
        }
        Err(_) => BrandsTemplate {
            brands: fetch_all_brands(&state).await,
            error_message: Some("حدث خطأ أثناء تعديل البراند، حاول مرة أخرى".to_string()),
            success_message: None,
            edit_brand: Some(old_brand),
        }
        .into_response(),
    }
}

/// Handles brand deletion requests.
///
/// Safely intercepts PostgreSQL Foreign Key Violations (`23503`) if the brand is bound
/// to existing products, providing a clear user feedback message instead of hard crashing.
pub async fn delete_brand_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Response {
    let result = sqlx::query!(r#"DELETE FROM brands WHERE id = $1"#, id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Redirect::to("/web/brands?ok=deleted").into_response(),
        // Error 23503: Foreign Key Constraint Violation (مرتبط بمنتجات أخرى)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            BrandsTemplate {
                brands: fetch_all_brands(&state).await,
                error_message: Some(
                    "لا يمكن حذف هذا البراند لأنه مرتبط بسجلات أخرى. قم بإزالتها أو نقلها أولاً."
                        .to_string(),
                ),
                success_message: None,
                edit_brand: None,
            }
            .into_response()
        }
        Err(_) => BrandsTemplate {
            brands: fetch_all_brands(&state).await,
            error_message: Some("حدث خطأ أثناء حذف البراند".to_string()),
            success_message: None,
            edit_brand: None,
        }
        .into_response(),
    }
}

// ============================================================================
// HANDLERS: LIVE SEARCH
// ============================================================================

/// Dynamic search handler returning a rendered Askama partial snippet.
/// Designed for live search / auto-complete integrations.
pub async fn search_brands_handler(
    State(state): State<AppState>,
    Query(query): Query<BrandSearchQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let q = query.q.trim();

    // إرجاع استجابة فارغة فوراً إن كان الاستعلام خالياً
    if q.is_empty() {
        return Ok(BrandSearchResultsTemplate {
            brands: vec![],
            query: String::new(),
        });
    }

    // إعداد نمط البحث غير حساس للحالة (Case-Insensitive) للغتين العربية والإنجليزية
    let search_pattern = format!("%{}%", q);

    let brands = sqlx::query_as!(
        BrandResponseDto,
        r#"
        SELECT id, name, name_ar, notes, created_at, updated_at
        FROM brands
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

    Ok(BrandSearchResultsTemplate {
        brands,
        query: q.to_string(),
    })
}