use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;
use askama::Template;
use askama_web::WebTemplate;
// use askama::Template; // أضفنا استيراد مكتبة Askama لإنشاء قوالب HTML

/// 1. Used for BOTH: Represents a brand record fetched from the database 
/// (used to display data back in JSON responses or passed into Askama HTML templates).
#[derive(Debug, Serialize, FromRow)]
pub struct BrandResponseDto {
    pub id: i32,
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 2. Used with ASKAMA (HTML Forms): Form struct for browser-based submissions with manual validation.
#[derive(Debug, Deserialize)]
pub struct CreateBrandForm {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

impl CreateBrandForm {
    /// Simple and direct validation logic for HTML form submissions
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("الاسم بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if self.name_ar.trim().is_empty() {
            return Err("الاسم بالعربية مطلوب".to_string());
        }
        if self.name.len() > 100 {
            return Err("اسم البراند طويل جداً".to_string());
        }
        Ok(())
    }
}

/// 3. Used with ASKAMA (HTML Forms): Form struct for browser-based updates with manual validation.
#[derive(Debug, Deserialize)]
pub struct UpdateBrandForm {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

impl UpdateBrandForm {
    /// Simple and direct validation logic for HTML update form submissions
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("الاسم بالإنجليزية مطلوب ولا يمكن تركه فارغاً".to_string());
        }
        if self.name_ar.trim().is_empty() {
            return Err("الاسم بالعربية مطلوب".to_string());
        }
        Ok(())
    }
}

/// 4. Used with ASKAMA (HTML Templates): Template struct that bridges Rust data and the HTML file.
/// This is required so Askama knows which HTML file to render and what data to inject into it.
#[derive(Template, WebTemplate)]
#[template(path = "brands.html")] // مسار ملف الـ HTML داخل مجلد templates
pub struct BrandsTemplate {
    pub brands: Vec<BrandResponseDto>,           // لعرض قائمة جميع البراندات
    pub error_message: Option<String>,
    pub success_message: Option<String>, // تأكد من وجود هذا السطر هنا
    pub edit_brand: Option<BrandResponseDto>,   // مخصص لتعْبئة نموذج التعديل إذا كان المستخدم يعدل براند معين
}

/// 5. Used with JSON API: DTO for creating a new brand via API requests (e.g., Postman).
#[derive(Debug, Deserialize)]
pub struct CreateBrandDto {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

/// 6. Used with JSON API: DTO for updating an existing brand via API requests (supports partial updates).
#[derive(Debug, Deserialize)]
pub struct UpdateBrandDto {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub notes: Option<String>,
}