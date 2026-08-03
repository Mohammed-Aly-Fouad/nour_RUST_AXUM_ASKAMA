use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

// ============================================================================
// SECTION 1: JSON API DTOs
// ============================================================================

/// DTO مشترك لعرض بيانات البراند (يُستخدم في JSON API وفي قوالب Askama معًا)
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct BrandResponseDto {
    pub id: i32,
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 1.1 Create Brand (POST)
// ---------------------------------------------------------------------------

/// DTO الخاص بإنشاء براند جديد عبر JSON API
#[derive(Debug, Deserialize)]
pub struct CreateBrandDto {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

impl CreateBrandDto {
    /// يتحقق من صحة البيانات المُرسلة لإنشاء براند جديد
    pub fn validate(&self) -> Result<(), (StatusCode, String)> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "English name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name.chars().count() > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                "English name must not exceed 100 characters".to_string(),
            ));
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name_ar.chars().count() > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic name must not exceed 100 characters".to_string(),
            ));
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 1.2 Update Brand (PATCH - partial update)
// ---------------------------------------------------------------------------

/// DTO الخاص بتحديث البراند (PATCH - تحديث جزئي)
/// كل الحقول اختيارية: لو الحقل None يعني المستخدم ما أرسله، فتبقى القيمة القديمة كما هي
#[derive(Debug, Deserialize)]
pub struct UpdateBrandDto {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub notes: Option<String>,
}

/// نسخة "مدموجة" تمثل القيم النهائية بعد دمج الجديد مع القديم
/// نستخدمها كمدخل موحّد لدالة الـ validate بدل تمرير حقول متفرقة
pub struct MergedBrandData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
}

impl<'a> MergedBrandData<'a> {
    /// يتحقق من صحة البيانات النهائية بعد الدمج
    pub fn validate(&self) -> Result<(), (StatusCode, String)> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "English name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name.chars().count() > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                "English name must not exceed 100 characters".to_string(),
            ));
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name_ar.chars().count() > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic name must not exceed 100 characters".to_string(),
            ));
        }

        Ok(())
    }
}

// ============================================================================
// SECTION 2: Web (HTML Forms + Askama Templates)
// ============================================================================

// ---------------------------------------------------------------------------
// 2.1 Create Brand (HTML Form)
// ---------------------------------------------------------------------------

/// نموذج إنشاء البراند عبر واجهة الويب (HTML Forms)
#[derive(Debug, Deserialize)]
pub struct CreateBrandForm {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

impl CreateBrandForm {
    pub fn validate(&self) -> Result<(), String> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err("الاسم بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name.chars().count() > 100 {
            return Err("اسم البراند طويل جداً (100 حرف كحد أقصى)".to_string());
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err("الاسم بالعربية مطلوب".to_string());
        }
        if trimmed_name_ar.chars().count() > 100 {
            return Err("اسم البراند بالعربية طويل جداً (100 حرف كحد أقصى)".to_string());
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 2.2 Update Brand (HTML Form)
// ---------------------------------------------------------------------------

/// نموذج تحديث البراند عبر واجهة الويب (HTML Forms)
#[derive(Debug, Deserialize)]
pub struct UpdateBrandForm {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

impl UpdateBrandForm {
    pub fn validate(&self) -> Result<(), String> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err("الاسم بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name.chars().count() > 100 {
            return Err("اسم البراند طويل جداً (100 حرف كحد أقصى)".to_string());
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err("الاسم بالعربية مطلوب".to_string());
        }
        if trimmed_name_ar.chars().count() > 100 {
            return Err("اسم البراند بالعربية طويل جداً (100 حرف كحد أقصى)".to_string());
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 2.3 Askama Template
// ---------------------------------------------------------------------------

/// قالب الـ Askama لعرض وإدارة البراندات في صفحات الويب
#[derive(Template, WebTemplate)]
#[template(path = "brands.html")]
pub struct BrandsTemplate {
    pub brands: Vec<BrandResponseDto>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub edit_brand: Option<BrandResponseDto>,
}