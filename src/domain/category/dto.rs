use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use askama::Template;
use askama_web::WebTemplate;
use serde::Deserializer;

/// 1. DTO لعرض بيانات الفئة (قادمة من قاعدة البيانات)
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

/// 2. نموذج إنشاء الفئة عبر واجهة الويب (HTML Forms)


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

#[derive(Deserialize, Serialize)]
pub struct CreateCategoryForm {
    pub name: String,
    pub name_ar: String,
    #[serde(deserialize_with = "empty_string_as_none", default)]
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
}

impl CreateCategoryForm {
    pub fn validate(&self, existing_categories: &[CategoryResponseDto]) -> Result<(), String> {
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
            let parent_category = existing_categories.iter().find(|cat| cat.id == pid);
            match parent_category {
                None => {
                    return Err("معرف الفئة الرئيسية (Parent ID) غير موجود".to_string());
                }
                Some(parent) => {
                    if parent.parent_id.is_some() {
                        return Err("لا يمكن اختيار فئة فرعية لتكون أب لفئة جديدة، يجب أن تكون الفئة المختارة رئيسية".to_string());
                    }
                }
            }
        }

        Ok(())
    }
}

/// 3. نموذج تحديث الفئة عبر واجهة الويب (HTML Forms)
#[derive(Deserialize, Serialize)]
pub struct UpdateCategoryForm {
    pub name: String,
    pub name_ar: String,
    #[serde(deserialize_with = "empty_string_as_none", default)]
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
}

impl UpdateCategoryForm {
    pub fn validate(
        &self, 
        current_category_id: i32, 
        existing_categories: &[CategoryResponseDto]
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
            if pid == current_category_id {
                return Err("لا يمكن تعيين الفئة كأب لنفسها".to_string());
            }

            let parent_category = existing_categories.iter().find(|cat| cat.id == pid);
            match parent_category {
                None => {
                    return Err("معرف الفئة الرئيسية (Parent ID) غير موجود".to_string());
                }
                Some(parent) => {
                    if parent.parent_id.is_some() {
                        return Err("لا يمكن اختيار فئة فرعية لتكون أب، يجب أن تكون الفئة الرئيسية جذعية".to_string());
                    }
                }
            }
        }

        Ok(())
    }
}

/// 4. قالب الـ Askama لعرض وإدارة الفئات في صفحات الويب
#[derive(Template, WebTemplate)]
#[template(path = "categories.html")]
pub struct CategoryTemplate {
    pub categories: Vec<CategoryResponseDto>,              
    pub error_message: Option<String>,
    pub success_message: Option<String>, 
    pub edit_category: Option<CategoryResponseDto>,  
}

/// 5. هيكل إنشاء فئة جديدة عبر الـ JSON API
#[derive(Deserialize, Serialize)]
pub struct CreateCategoryApiDto {
    pub name: String,
    pub name_ar: String,
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
}

impl CreateCategoryApiDto {
    pub fn validate(&self, existing_categories: &[CategoryResponseDto]) -> Result<(), String> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err("English name is required and cannot be empty".to_string());
        }
        if trimmed_name.chars().count() > 50 {
            return Err("English name must not exceed 50 characters".to_string());
        }

        let trimmed_name_ar = self.name_ar.trim();
        if trimmed_name_ar.is_empty() {
            return Err("Arabic name is required and cannot be empty".to_string());
        }
        if trimmed_name_ar.chars().count() > 50 {
            return Err("Arabic name must not exceed 50 characters".to_string());
        }

        if let Some(pid) = self.parent_id {
            let parent_category = existing_categories.iter().find(|cat| cat.id == pid);
            match parent_category {
                None => return Err("Parent ID does not exist".to_string()),
                Some(parent) => {
                    if parent.parent_id.is_some() {
                        return Err("Cannot select a sub-category as a parent. Must be a top-level category.".to_string());
                    }
                }
            }
        }

        Ok(())
    }
}

/// 6. هيكل التحديث الجزئي لفئة عبر الـ JSON API (PATCH / PUT)
#[derive(Debug, Deserialize)]
pub struct UpdateCategoryApiDto {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub parent_id: Option<i32>,
    pub notes: Option<String>,
}

impl UpdateCategoryApiDto {
    pub fn validate(&self, current_category_id: i32, existing_categories: &[CategoryResponseDto]) -> Result<(), String> {
        if let Some(ref name) = self.name {
            let trimmed_name = name.trim();
            if trimmed_name.is_empty() {
                return Err("English name cannot be empty".to_string());
            }
            if trimmed_name.chars().count() > 50 {
                return Err("English name must not exceed 50 characters".to_string());
            }
        }

        if let Some(ref name_ar) = self.name_ar {
            let trimmed_name_ar = name_ar.trim();
            if trimmed_name_ar.is_empty() {
                return Err("Arabic name cannot be empty".to_string());
            }
            if trimmed_name_ar.chars().count() > 50 {
                return Err("Arabic name must not exceed 50 characters".to_string());
            }
        }

        if let Some(pid) = self.parent_id {
            if pid == current_category_id {
                return Err("Cannot set a category as a parent to itself".to_string());
            }

            let parent_category = existing_categories.iter().find(|cat| cat.id == pid);
            match parent_category {
                None => {
                    return Err("Parent ID does not exist".to_string());
                }
                Some(parent) => {
                    if parent.parent_id.is_some() {
                        return Err("Cannot select a sub-category as a parent. Must be a top-level category.".to_string());
                    }
                }
            }
        }

        Ok(())
    }
}