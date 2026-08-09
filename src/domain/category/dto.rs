use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

// ============================================================================
// HELPERS FOR DESERIALIZATION
// ============================================================================

/// يحوّل حقل رقمي فارغ (سلسلة نصية فارغة "") من HTML Form إلى None
pub fn empty_number_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OptionNum<T> {
        Num(T),
        Str(String),
        None,
    }

    match OptionNum::<T>::deserialize(deserializer)? {
        OptionNum::Num(n) => Ok(Some(n)),
        OptionNum::Str(s) if s.trim().is_empty() => Ok(None),
        OptionNum::Str(s) => s.trim().parse::<T>().map(Some).map_err(serde::de::Error::custom),
        OptionNum::None => Ok(None),
    }
}

/// يحوّل حقل ملاحظات نصي فارغ "" من HTML Form إلى None
pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}

// ============================================================================
// SECTION 1: JSON API DTOs
// ============================================================================

#[derive(Deserialize, Serialize, FromRow, Clone)]
pub struct CategoryResponseDto {
    pub id: i64,
    pub name: String,
    pub name_ar: String,
    pub parent_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CategoryTreeDto {
    pub id: i64,
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub children: Vec<CategoryTreeDto>,
}

impl CategoryTreeDto {
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

    pub fn build_tree(flat_categories: Vec<CategoryResponseDto>) -> Vec<CategoryTreeDto> {
        let mut children_map: HashMap<i64, Vec<&CategoryResponseDto>> = HashMap::new();
        let mut roots: Vec<&CategoryResponseDto> = Vec::new();

        for cat in &flat_categories {
            match cat.parent_id {
                Some(pid) => children_map.entry(pid).or_default().push(cat),
                None => roots.push(cat),
            }
        }

        roots
            .into_iter()
            .map(|root| Self::build_node(root, &children_map))
            .collect()
    }

    fn build_node(
        cat: &CategoryResponseDto,
        children_map: &HashMap<i64, Vec<&CategoryResponseDto>>,
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

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryApiDto {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub parent_id: Option<i64>,
    pub notes: Option<String>,
}

pub struct MergedCategoryData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
    pub parent_id: Option<i64>,
}

impl<'a> MergedCategoryData<'a> {
    pub async fn validate(
        &self,
        current_category_id: i64,
        old_parent_id: Option<i64>,
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

        // 🎯 فحص تكرار الاسم بالإنجليزية عبر SQL (Async API)
        let name_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM categories
                WHERE LOWER(TRIM(name)) = LOWER($1) AND id != $2
            ) AS "exists!"
            "#,
            trimmed_name,
            current_category_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if name_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "اسم الفئة بالإنجليزية موجود بالفعل".to_string(),
            ));
        }

        // 🎯 فحص تكرار الاسم بالعربية عبر SQL (Async API)
        let name_ar_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM categories
                WHERE TRIM(name_ar) = $1 AND id != $2
            ) AS "exists!"
            "#,
            trimmed_name_ar,
            current_category_id
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if name_ar_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "اسم الفئة بالعربية موجود بالفعل".to_string(),
            ));
        }

        if let Some(pid) = self.parent_id {
            if pid == current_category_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "لا يمكن تعيين الفئة كأب لنفسها".to_string(),
                ));
            }

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
                let msg = "معرف الفئة الرئيسية غير موجود، أو أنها ليست فئة جذعية".to_string();
                return Err((StatusCode::BAD_REQUEST, msg));
            }

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
                    let msg = "لا يمكن تحويل هذه الفئة إلى فئة فرعية لأن لديها فئات فرعية تابعة لها".to_string();
                    return Err((StatusCode::BAD_REQUEST, msg));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryApiDto {
    pub name: String,
    pub name_ar: String,
    pub parent_id: Option<i64>,
    pub notes: Option<String>,
}

impl CreateCategoryApiDto {
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

        // 🎯 فحص تكرار الاسم بالإنجليزية عند الإنشاء (API)
        let name_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM categories
                WHERE LOWER(TRIM(name)) = LOWER($1)
            ) AS "exists!"
            "#,
            trimmed_name
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if name_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "English category name already exists".to_string(),
            ));
        }

        // 🎯 فحص تكرار الاسم بالعربية عند الإنشاء (API)
        let name_ar_exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM categories
                WHERE TRIM(name_ar) = $1
            ) AS "exists!"
            "#,
            trimmed_name_ar
        )
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if name_ar_exists {
            return Err((
                StatusCode::BAD_REQUEST,
                "Arabic category name already exists".to_string(),
            ));
        }

        if let Some(pid) = self.parent_id {
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
                let msg = "Parent ID does not exist, or is not a top-level category".to_string();
                return Err((StatusCode::BAD_REQUEST, msg));
            }
        }

        Ok(())
    }
}

// ============================================================================
// SECTION 2: Web (HTML Forms + Askama Templates)
// ============================================================================

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryForm {
    pub name: String,
    pub name_ar: String,

    // Uses i32 helper
    #[serde(default, deserialize_with = "empty_number_as_none")]
    pub parent_id: Option<i64>,

    // Uses String helper
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

impl CreateCategoryForm {
    pub fn validate(&self, existing_categories: &[CategoryResponseDto]) -> Result<(), String> {
        let merged = MergedCategoryFormData {
            name: &self.name,
            name_ar: &self.name_ar,
            parent_id: self.parent_id,
        };
        merged.validate(None, existing_categories)
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryForm {
    pub name: String,
    pub name_ar: String,

    #[serde(default, deserialize_with = "empty_number_as_none")]
    pub parent_id: Option<i64>,

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

impl UpdateCategoryForm {
    pub fn validate(
        &self,
        current_category_id: i64,
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

pub struct MergedCategoryFormData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
    pub parent_id: Option<i64>,
}

impl<'a> MergedCategoryFormData<'a> {
    pub fn validate(
        &self,
        current_category_id: Option<i64>,
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

        // 🎯 1. فحص التكرار للاسم بالإنجليزية (In-Memory Case-insensitive + Trim)
        let name_exists = existing_categories.iter().any(|cat| {
            Some(cat.id) != current_category_id
                && cat.name.trim().eq_ignore_ascii_case(trimmed_name)
        });
        if name_exists {
            return Err("اسم الفئة بالإنجليزية موجود بالفعل".to_string());
        }

        // 🎯 2. فحص التكرار للاسم بالعربية
        let name_ar_exists = existing_categories.iter().any(|cat| {
            Some(cat.id) != current_category_id
                && cat.name_ar.trim() == trimmed_name_ar
        });
        if name_ar_exists {
            return Err("اسم الفئة بالعربية موجود بالفعل".to_string());
        }

        // 🎯 3. التحقق من صلاحية الفئة الأب + حظر الأبناء
        if let Some(pid) = self.parent_id {
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

            // إذا كان هذا التعديل لفئة حالية وكانت فئة جذعية، نتأكد أنها لا تملك أبناء قبل تحويلها لفئة فرعية
            if let Some(cid) = current_category_id {
                let current_cat = existing_categories.iter().find(|c| c.id == cid);
                if let Some(cat) = current_cat {
                    if cat.parent_id.is_none() {
                        let has_children = existing_categories.iter().any(|c| c.parent_id == Some(cid));
                        if has_children {
                            return Err(
                                "لا يمكن تحويل هذه الفئة إلى فئة فرعية لأن لديها فئات فرعية تابعة لها"
                                    .to_string(),
                            );
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CategoryRow {
    pub id: i64,
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub parent_name: Option<String>,
}

impl CategoryRow {
    pub fn build_rows(categories: &[CategoryResponseDto]) -> Vec<CategoryRow> {
        let id_to_name: HashMap<i64, &str> =
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

#[derive(Template, WebTemplate)]
#[template(path = "categories.html")]
pub struct CategoryTemplate {
    pub categories: Vec<CategoryRow>,
    pub root_categories: Vec<CategoryResponseDto>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub edit_category: Option<CategoryResponseDto>,
}

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