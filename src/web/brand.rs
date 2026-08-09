use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;

use crate::domain::brand::dto::{
    BrandResponseDto, BrandsTemplate, CreateBrandForm, MergedBrandFormData, UpdateBrandForm,
};
use crate::state::AppState;

/// Configures and returns the sub-router for all browser-based Askama HTML endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        // GET /brands -> Render main page with list & form | POST /brands -> Handle creation with validation
        .route("/", get(render_brands_page).post(create_brand_web))
        // GET /brands/edit/{id} -> Render page with specific brand loaded for editing
        .route("/edit/{id}", get(edit_brand_page))
        // POST /brands/update/{id} -> Handle update submission with validation
        .route("/update/{id}", post(update_brand_web))
        // GET /brands/delete/{id} -> Handle brand deletion
        .route("/delete/{id}", post(delete_brand_web))
}

// ============================================================================
// Query params used to carry a "flash message" across a redirect.
//
// ليه احتجنا للحاجة دي؟
// بعد أي عملية ناجحة (إنشاء/تعديل) بنعمل Redirect للصفحة الرئيسية. الـ Redirect
// معناه المتصفح بيبعت GET request جديد تمامًا — أي BrandsTemplate كنا بنبنيه
// في نفس الـ response بيضيع، ومعاه success_message. عشان نقدر نعرض
// "تم الحفظ بنجاح" حتى بعد الـ redirect، بنمرر إشارة بسيطة في الـ query string
// (?ok=1) وبنقرأها في render_brands_page.
//
// ده أبسط حل ممكن (من غير Session/Cookies). لو احتجت رسالة نصية مخصصة لاحقًا
// (مش بس "تم الحفظ")، الخطوة التالية المنطقية هي flash messages عبر session.
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub ok: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers: shared queries to avoid repeating the same SELECT everywhere
// ---------------------------------------------------------------------------

/// يجلب كل البراندات مرتبة تنازليًا حسب الـ ID.
/// عند فشل الاستعلام بيرجّع قائمة فاضية بدل ما يوقف الصفحة بالكامل — لأن
/// صفحة الويب لازم تفضل قابلة للعرض حتى لو حصل خطأ مؤقت في قاعدة البيانات.
async fn fetch_all_brands(state: &AppState) -> Vec<BrandResponseDto> {
    sqlx::query_as!(
        BrandResponseDto,
        r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands ORDER BY id DESC"#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

/// يجلب براند واحد بالـ ID (يُستخدم لتحميل بيانات فورم التعديل مسبقًا).
async fn fetch_brand_by_id(state: &AppState, id: i64) -> Option<BrandResponseDto> {
    sqlx::query_as!(
        BrandResponseDto,
        r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(None)
}

// ==================== 1. Render main page ====================

/// Renders the main HTML page containing the brand list and creation form.
/// Reads `?ok=...` from the query string to show a one-time success message
/// after a redirect (see the FlashParams note above).
pub async fn render_brands_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> BrandsTemplate {
    let success_message = match params.ok.as_deref() {
        Some("created") => Some("تم إضافة البراند بنجاح".to_string()),
        Some("updated") => Some("تم تعديل البراند بنجاح".to_string()),
        _ => None,
    };

    BrandsTemplate {
        brands: fetch_all_brands(&state).await,
        error_message: None,
        success_message,
        edit_brand: None,
    }
}

// ==================== 2. Create Brand ====================

/// Handles web form submission for creating a new brand, with full validation
/// and error handling mirroring the JSON API's `create_brand`.
pub async fn create_brand_web(
    State(state): State<AppState>,
    Form(form): Form<CreateBrandForm>,
) -> axum::response::Response {
    // 1. Validate the incoming form data
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

    // 2. Insert into the database
    let result = sqlx::query!(
        r#"INSERT INTO brands (name, name_ar, notes) VALUES ($1, $2, $3)"#,
        trimmed_name,
        trimmed_name_ar,
        form.notes
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/brands?ok=created").into_response(),
        // 23505: unique_violation - نفس الكود اللي بيتحقق منه الـ API بالظبط
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

// ==================== 3. Edit page (GET) ====================

/// Renders the page with a specific brand pre-loaded in the edit form.
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

// ==================== 4. Update Brand ====================

/// Handles web form submission for updating an existing brand.
///
/// ملاحظة: الفورم هنا بيرجّع كل حقوله دايمًا (عكس PATCH في الـ API)، فمفيش
/// حاجة فعلية "تتدمج" مع قيم قديمة. لكن بنستخدم MergedBrandFormData::validate
/// برضو، عشان نفس منطق التحقق (الرسائل + الحدود) يكون مصدره مكان واحد بس،
/// بدل ما نكرره هنا تاني بشكل منفصل عن UpdateBrandForm::validate.
pub async fn update_brand_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateBrandForm>,
) -> axum::response::Response {
    // 1. تأكد إن البراند موجود أصلًا قبل أي حاجة
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

    // 2. تحقق من صحة البيانات القادمة (عبر نفس مسار MergedBrandFormData)
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

    // 3. نفّذ التحديث في قاعدة البيانات
    let result = sqlx::query!(
        r#"UPDATE brands SET name = $1, name_ar = $2, notes = $3, updated_at = now() WHERE id = $4"#,
        trimmed_name,
        trimmed_name_ar,
        form.notes,
        id
    )
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => Redirect::to("/web/brands?ok=updated").into_response(),
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

// ==================== 5. Delete Brand ====================

/// Handles brand deletion from the web interface.
/// Unlike the previous version, this no longer swallows the error silently:
/// if the brand has related records (e.g. products), we show a clear message
/// instead of failing quietly.
pub async fn delete_brand_web(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    let result = sqlx::query!(r#"DELETE FROM brands WHERE id = $1"#, id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Redirect::to("/web/brands").into_response(),
        // 23503: foreign key violation - نفس المنطق بالظبط اللي في الـ API
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