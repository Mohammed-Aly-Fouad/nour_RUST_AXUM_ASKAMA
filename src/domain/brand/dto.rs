use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

// ============================================================================
// SECTION 1: JSON API DTOs
// ============================================================================

/// Shared DTO for displaying brand data (used in both the JSON API and Askama templates)
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
//
// ملاحظة تعليمية مهمة قبل ما نكمل:
// في الـ API استخدمنا PATCH مع حقول Optional + خطوة "دمج" (MergedBrandData)
// لأن العميل (Postman / Frontend عبر fetch) ممكن يبعت جزء من الحقول بس.
//
// أما في فورم HTML عادي (<form method="post">)، كل حقول الفورم بترجع مع كل
// submit (حتى لو فاضية) — مفيش مفهوم "حقل غير موجود" بنفس المعنى. فالفورم
// دايمًا هيعرض القيم الحالية للمستخدم قبل التعديل، وهو يرجّعها كلها تاني.
// لذلك name و name_ar بيفضلوا إجباريين (String) مش Optional هنا.
//
// لكن عشان نحافظ على نفس *تركيب* الكود (خام -> دمج -> تحقق)، ضفنا
// MergedBrandFormData بنفس فكرة MergedBrandData في القسم الأول.
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
    /// يتحقق من صحة بيانات إنشاء البراند القادمة من الفورم
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
/// الحقول هنا إجبارية (وليست Optional) لأن الفورم بيرجّع كل حقوله دايمًا،
/// عكس الـ JSON PATCH في قسم الـ API. راجع الملاحظة أعلى القسم للتفاصيل.
#[derive(Debug, Deserialize)]
pub struct UpdateBrandForm {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

/// نسخة "مدموجة" من بيانات التحديث القادمة من فورم HTML - نفس فكرة
/// MergedBrandData في قسم الـ API، بنستخدمها كمدخل موحّد لدالة الـ validate
/// بدل ما نكرر التحقق جوه UpdateBrandForm مباشرة. الفايدة عمليًا هنا إنك
/// لو حبيت مستقبلًا تسمح بتعديل جزئي حتى من الفورم (مثلاً عبر JS/AJAX)،
/// تقدر تبني MergedBrandFormData من (القيم الجديدة المرسلة + القيم القديمة
/// من قاعدة البيانات) من غير ما تلمس منطق التحقق نفسه.
pub struct MergedBrandFormData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
}

impl<'a> MergedBrandFormData<'a> {
    /// يتحقق من صحة البيانات النهائية بعد الدمج (نفس رسائل UpdateBrandForm)
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

impl UpdateBrandForm {
    /// يتحقق من صحة بيانات تحديث البراند القادمة من الفورم
    /// (بيمرّر البيانات لـ MergedBrandFormData::validate للحفاظ على تركيب موحّد)
    pub fn validate(&self) -> Result<(), String> {
        let merged = MergedBrandFormData {
            name: &self.name,
            name_ar: &self.name_ar,
        };
        merged.validate()
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

pub mod filters {
    use askama::Values;

    /// يرجّع أول حرف من اسم البراند (Capital) لعرضه داخل الـ avatar الدائري
    #[askama::filter_fn]
    pub fn first_letter(name: &str, _values: &dyn Values) -> askama::Result<String> {
        Ok(name
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "?".to_string()))
    }

    /// يولّد لون خلفية ثابت للـ avatar بناءً على اسم البراند — نفس الاسم
    /// دايمًا هياخد نفس اللون (مش عشوائي في كل تحميل للصفحة)
    #[askama::filter_fn]
    pub fn initial_color(name: &str, _values: &dyn Values) -> askama::Result<String> {
        const PALETTE: [&str; 6] =
            ["#0E7C66", "#2563EB", "#D97706", "#7C3AED", "#DB2777", "#0891B2"];
        let sum: u32 = name.bytes().map(|b| b as u32).sum();
        Ok(PALETTE[sum as usize % PALETTE.len()].to_string())
    }
}