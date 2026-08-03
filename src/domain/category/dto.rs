use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

// ============================================================================
// SECTION 1: JSON API DTOs
// ============================================================================

/// DTO لعرض بيانات الفئة (قادمة من قاعدة البيانات)
#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct CategoryResponseDto {
    pub id: i32,
    pub name: String,
    pub name_ar: String,
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// 1.1 List Categories as Tree
// ---------------------------------------------------------------------------

/// DTO يمثل الفئة مع فروعها بشكل متداخل (Nested Tree)
/// يُستخدم فقط للعرض (Response) - نبنيه من CategoryResponseDto بعد جلب البيانات من DB
#[derive(Debug, Serialize, Clone)]
pub struct CategoryTreeDto {
    pub id: i32,
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub children: Vec<CategoryTreeDto>,
}

impl CategoryTreeDto {
    /// يبني من CategoryResponseDto واحدة (بدون أطفال بعد)
    fn from_flat(cat: &CategoryResponseDto) -> Self {
        CategoryTreeDto {
            id: cat.id,
            name: cat.name.clone(),
            name_ar: cat.name_ar.clone(),
            notes: cat.notes.clone(),
            created_at: cat.created_at,
            updated_at: cat.updated_at,
            children: Vec::new(),
        }
    }

    /// يحوّل قائمة مسطّحة (flat list) من الفئات إلى شجرة متداخلة
    /// الخوارزمية: O(n) - مرة واحدة لتجميع الأبناء حسب parent_id، ومرة لبناء الشجرة
    pub fn build_tree(flat_categories: Vec<CategoryResponseDto>) -> Vec<CategoryTreeDto> {
        // 1. تجميع كل فئة تحت parent_id الخاص بها
        let mut children_map: HashMap<i32, Vec<&CategoryResponseDto>> = HashMap::new();
        let mut roots: Vec<&CategoryResponseDto> = Vec::new();

        for cat in &flat_categories {
            match cat.parent_id {
                Some(pid) => children_map.entry(pid).or_default().push(cat),
                None => roots.push(cat),
            }
        }

        // 2. بناء الشجرة بدءًا من الفئات الجذعية (roots)
        roots
            .into_iter()
            .map(|root| Self::build_node(root, &children_map))
            .collect()
    }

    /// يبني عقدة واحدة (فئة) مع كل أبنائها بشكل تعاودي (recursive)
    /// ملاحظة: بما أن الهيكل عندك مستواه اثنين فقط (أب/فرع)،
    /// الاستدعاء التعاودي هنا آمن ولن يسبب حلقة لا نهائية
    fn build_node(
        cat: &CategoryResponseDto,
        children_map: &HashMap<i32, Vec<&CategoryResponseDto>>,
    ) -> CategoryTreeDto {
        let mut node = Self::from_flat(cat);

        if let Some(children) = children_map.get(&cat.id) {
            node.children = children
                .iter()
                .map(|child| Self::build_node(child, children_map))
                .collect();
        }

        node
    }
}

// ---------------------------------------------------------------------------
// 1.2 Update Category (PATCH)
// ---------------------------------------------------------------------------

/// DTO الخاص بتحديث الفئة (PATCH - تحديث جزئي)
/// كل الحقول اختيارية: لو الحقل None يعني المستخدم ما أرسله، فتبقى القيمة القديمة كما هي
#[derive(Debug, Deserialize)]
pub struct UpdateCategoryApiDto {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
}

/// نسخة "مدموجة" تمثل القيم النهائية بعد دمج الجديد مع القديم
/// نستخدمها كمدخل موحّد لدالة الـ validate بدل تمرير حقول متفرقة
pub struct MergedCategoryData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
    pub parent_id: Option<i32>,
}

impl<'a> MergedCategoryData<'a> {
    /// يتحقق من صحة البيانات النهائية بعد الدمج
    /// current_category_id: لمنع الفئة من أن تكون أب لنفسها، وللتحقق من وجود فروع تابعة لها
    /// old_parent_id: قيمة parent_id القديمة (قبل التعديل) - نحتاجها لمعرفة هل فعلاً نغيّر الفئة من "أب" إلى "ابن"
    /// نستخدم استعلامات DB مباشرة (EXISTS) بدل تحميل كل الفئات - أفضل أداءً
    pub async fn validate(
        &self,
        current_category_id: i32,
        old_parent_id: Option<i32>,
        pool: &sqlx::PgPool,
    ) -> Result<(), (StatusCode, String)> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "الاسم بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string(),
            ));
        }
        if trimmed_name.chars().count() > 50 {
            return Err((
                StatusCode::BAD_REQUEST,
                "اسم الفئة بالإنجليزية يجب ألا يتجاوز 50 حرفاً".to_string(),
            ));
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "الاسم بالعربية مطلوب ولا يمكن تركه فارغاً".to_string(),
            ));
        }
        if trimmed_name_ar.chars().count() > 50 {
            return Err((
                StatusCode::BAD_REQUEST,
                "اسم الفئة بالعربية يجب ألا يتجاوز 50 حرفاً".to_string(),
            ));
        }

        if let Some(pid) = self.parent_id {
            // 1. لا يمكن للفئة أن تكون أباً لنفسها
            if pid == current_category_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "لا يمكن تعيين الفئة كأب لنفسها".to_string(),
                ));
            }

            // 2. الأب المُختار يجب أن يكون موجوداً وأن يكون فئة جذعية (parent_id = NULL)
            let parent_is_valid = sqlx::query_scalar!(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM categories
                    WHERE id = $1 AND parent_id IS NULL
                ) AS "exists!"
                "#,
                pid
            )
            .fetch_one(pool)
            .await
            .unwrap_or(false);

            if !parent_is_valid {
                let msg = "معرف الفئة الرئيسية غير موجود، أو أنها ليست فئة جذعية (لا يمكن اختيار فئة فرعية كأب)".to_string();
                return Err((StatusCode::BAD_REQUEST, msg));
            }

            // 3. لو الفئة كانت "جذعية" (أب) قبل التعديل، وصار عندها الآن أب جديد (تتحول لفئة فرعية)
            //    نتأكد ما عندها فئات فرعية تابعة لها، لأن هذا يسبب تعارض في الهيكل (فرع يتبع لفرع)
            if old_parent_id.is_none() {
                let has_children = sqlx::query_scalar!(
                    r#"
                    SELECT EXISTS(
                        SELECT 1 FROM categories
                        WHERE parent_id = $1
                    ) AS "exists!"
                    "#,
                    current_category_id
                )
                .fetch_one(pool)
                .await
                .unwrap_or(false);

                if has_children {
                    let msg = "لا يمكن تحويل هذه الفئة إلى فئة فرعية لأن لديها فئات فرعية تابعة لها. يجب نقل أو حذف الفئات الفرعية التابعة أولاً".to_string();
                    return Err((StatusCode::BAD_REQUEST, msg));
                }
            }
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 1.3 Create Category (POST)
// ---------------------------------------------------------------------------

/// DTO الخاص بإنشاء فئة جديدة (POST)
#[derive(Debug, Deserialize)]
pub struct CreateCategoryApiDto {
    pub name: String,
    pub name_ar: String,
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
}

impl CreateCategoryApiDto {
    /// يتحقق من صحة البيانات المُرسلة لإنشاء فئة جديدة
    /// نستخدم استعلام DB مباشر (EXISTS) بدل تحميل كل الفئات - أفضل أداءً،
    /// وخصوصًا هنا لأننا لا نحتاج أي بيانات أخرى غير التحقق من صلاحية الأب
    pub async fn validate(&self, pool: &sqlx::PgPool) -> Result<(), (StatusCode, String)> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "English name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name.chars().count() > 50 {
            return Err((
                StatusCode::BAD_REQUEST,
                "English name must not exceed 50 characters".to_string(),
            ));
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic name is required and cannot be empty".to_string(),
            ));
        }
        if trimmed_name_ar.chars().count() > 50 {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic name must not exceed 50 characters".to_string(),
            ));
        }

        if let Some(pid) = self.parent_id {
            // الأب المُختار يجب أن يكون موجوداً وأن يكون فئة جذعية (parent_id = NULL)
            let parent_is_valid = sqlx::query_scalar!(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM categories
                    WHERE id = $1 AND parent_id IS NULL
                ) AS "exists!"
                "#,
                pid
            )
            .fetch_one(pool)
            .await
            .unwrap_or(false);

            if !parent_is_valid {
                let msg = "Parent ID does not exist, or is not a top-level category (cannot select a sub-category as parent)".to_string();
                return Err((StatusCode::BAD_REQUEST, msg));
            }
        }

        Ok(())
    }
}

// ============================================================================
// SECTION 2: Web (HTML Forms + Askama Templates)
// ============================================================================

/// يحوّل حقل نموذج HTML فارغ (سلسلة نصية فارغة) إلى None بدل محاولة تحويله لرقم
/// مفيد لأن حقول <select> أو <input> في نماذج HTML ترسل "" وليس null عند عدم الاختيار
pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => match s.parse::<i32>() {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(serde::de::Error::custom(e)),
        },
    }
}

// ---------------------------------------------------------------------------
// 2.1 Create Category (HTML Form)
// ---------------------------------------------------------------------------

/// نموذج إنشاء الفئة عبر واجهة الويب (HTML Forms)
#[derive(Deserialize, Serialize)]
pub struct CreateCategoryForm {
    pub name: String,
    pub name_ar: String,
    #[serde(deserialize_with = "empty_string_as_none", default)]
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
}

impl CreateCategoryForm {
    /// يفوّض التحقق لـ MergedCategoryFormData (current_category_id = None لأن
    /// الفئة الجديدة لسه مالهاش id، فمفيش داعي للتحقق من "أب لنفسه")
    pub fn validate(&self, existing_categories: &[CategoryResponseDto]) -> Result<(), String> {
        let merged = MergedCategoryFormData {
            name: &self.name,
            name_ar: &self.name_ar,
            parent_id: self.parent_id,
        };
        merged.validate(None, existing_categories)
    }
}

// ---------------------------------------------------------------------------
// 2.2 Update Category (HTML Form)
// ---------------------------------------------------------------------------

/// نموذج تحديث الفئة عبر واجهة الويب (HTML Forms)
#[derive(Deserialize, Serialize)]
pub struct UpdateCategoryForm {
    pub name: String,
    pub name_ar: String,
    #[serde(deserialize_with = "empty_string_as_none", default)]
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
}

impl UpdateCategoryForm {
    /// يفوّض التحقق لـ MergedCategoryFormData (current_category_id = Some(id)
    /// عشان نمنع الفئة من إن تبقى أب لنفسها)
    pub fn validate(
        &self,
        current_category_id: i32,
        existing_categories: &[CategoryResponseDto],
    ) -> Result<(), String> {
        let merged = MergedCategoryFormData {
            name: &self.name,
            name_ar: &self.name_ar,
            parent_id: self.parent_id,
        };
        merged.validate(Some(current_category_id), existing_categories)
    }
}

/// نسخة "مدموجة" من بيانات الفئة القادمة من فورم HTML (سواء إنشاء أو تعديل).
/// نفس فكرة MergedBrandFormData بالظبط: طبقة واحدة تجمع منطق التحقق المشترك
/// بدل ما يتكرر حرفيًا في CreateCategoryForm و UpdateCategoryForm كل على حدة.
///
/// هنا الفايدة حقيقية أكتر من حالة البراند: منطق التحقق من parent_id (وجوده،
/// كونه فئة جذعية، ومنع الفئة من أن تكون أب لنفسها) كان متكرر بالحرف الواحد
/// في الملفين - دلوقتي مكتوب مرة واحدة بس.
pub struct MergedCategoryFormData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
    pub parent_id: Option<i32>,
}

impl<'a> MergedCategoryFormData<'a> {
    /// current_category_id:
    /// - None    -> حالة الإنشاء (فئة جديدة، مفيش id لسه)
    /// - Some(id) -> حالة التعديل (نمنع الفئة من اختيار نفسها كأب)
    pub fn validate(
        &self,
        current_category_id: Option<i32>,
        existing_categories: &[CategoryResponseDto],
    ) -> Result<(), String> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err("الاسم بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name.chars().count() > 50 {
            return Err("اسم الفئة بالإنجليزية يجب ألا يتجاوز 50 حرفاً".to_string());
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err("الاسم بالعربية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if trimmed_name_ar.chars().count() > 50 {
            return Err("اسم الفئة بالعربية يجب ألا يتجاوز 50 حرفاً".to_string());
        }

        if let Some(pid) = self.parent_id {
            // لا يمكن للفئة أن تكون أباً لنفسها (بيسري بس وقت التعديل)
            if Some(pid) == current_category_id {
                return Err("لا يمكن تعيين الفئة كأب لنفسها".to_string());
            }

            match existing_categories.iter().find(|cat| cat.id == pid) {
                None => {
                    return Err("معرف الفئة الرئيسية (Parent ID) غير موجود".to_string());
                }
                Some(parent) if parent.parent_id.is_some() => {
                    return Err(
                        "لا يمكن اختيار فئة فرعية لتكون أب، يجب أن تكون الفئة المختارة جذعية"
                            .to_string(),
                    );
                }
                Some(_) => {}
            }
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// 2.3 View Model: صف جاهز للعرض في جدول الفئات
// ---------------------------------------------------------------------------

/// صف معروض في جدول الفئات - نفس بيانات CategoryResponseDto، لكن مع اسم
/// الفئة الأب جاهز كنص (بدل ما التمبلت يدوّر عليه بنفسه بين كل الفئات).
/// الحساب بيتم في Rust مرة واحدة وقت التحضير، عشان التمبلت يفضل "غبي" وبسيط
/// ومحتاجش فلاتر مخصصة معقدة بتدور جوه قايمة.
#[derive(Debug, Clone)]
pub struct CategoryRow {
    pub id: i32,
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    /// None  -> فئة جذعية (هتتعرض كـ badge "فئة رئيسية")
    /// Some(name) -> اسم الفئة الأب
    pub parent_name: Option<String>,
}

impl CategoryRow {
    /// يحوّل قائمة مسطّحة من CategoryResponseDto إلى صفوف جاهزة للعرض.
    /// بيحل اسم كل أب مرة واحدة عبر HashMap (O(n) بدل ما ندوّر جوه القايمة
    /// لكل فئة على حدة وناخد O(n^2))
    pub fn build_rows(categories: &[CategoryResponseDto]) -> Vec<CategoryRow> {
        let id_to_name: HashMap<i32, &str> =
            categories.iter().map(|c| (c.id, c.name.as_str())).collect();

        categories
            .iter()
            .map(|c| CategoryRow {
                id: c.id,
                name: c.name.clone(),
                name_ar: c.name_ar.clone(),
                notes: c.notes.clone(),
                created_at: c.created_at,
                parent_name: c
                    .parent_id
                    .and_then(|pid| id_to_name.get(&pid).map(|n| n.to_string())),
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// 2.4 Askama Template
// ---------------------------------------------------------------------------

/// قالب الـ Askama لعرض وإدارة الفئات في صفحات الويب
#[derive(Template, WebTemplate)]
#[template(path = "categories-fetch.html")]
pub struct CategoryTemplate {
    /// صفوف الجدول (جاهزة للعرض، فيها اسم الأب محسوب مسبقًا)
    pub categories: Vec<CategoryRow>,
    /// الفئات الجذعية بس - تُستخدم لملء قائمة اختيار "الفئة الرئيسية" في الفورم
    pub root_categories: Vec<CategoryResponseDto>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub edit_category: Option<CategoryResponseDto>,
}

// ---------------------------------------------------------------------------
// 2.5 Custom Askama filters (نفس فكرة filters بتاعة البراند، للـ avatar الملوّن)
// ---------------------------------------------------------------------------

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




