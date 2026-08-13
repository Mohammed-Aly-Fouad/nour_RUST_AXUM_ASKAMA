use askama::Template;
use askama_web::WebTemplate;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

// Unused imports cleared (e.g., crate::domain::brand::model::Brand)

// ============================================================================
// SECTION 1: SHARED RESPONSE DTOS
// ============================================================================

/// Shared Data Transfer Object representing a single Brand entity.
///
/// Used seamlessly across both standard JSON REST APIs and HTML Askama templates.
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct BrandResponseDto {
    pub id: i64,
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// SECTION 2: JSON REST API DTOS & VALIDATION
// ============================================================================

// ---------------------------------------------------------------------------
// 2.1 Create Brand (POST Request Body)
// ---------------------------------------------------------------------------

/// DTO for creating a brand via JSON API (`POST /api/v1/brands`).
#[derive(Debug, Deserialize)]
pub struct CreateBrandDto {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

impl CreateBrandDto {
    /// Validates the JSON creation payload fields.
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
// 2.2 Update Brand (PATCH Partial Update)
// ---------------------------------------------------------------------------

/// DTO for updating a brand via JSON API (`PATCH /api/v1/brands/{id}`).
/// All fields are optional to support partial updates.
#[derive(Debug, Deserialize)]
pub struct UpdateBrandDto {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub notes: Option<String>,
}

/// Represents the merged dataset after combining optional JSON updates with DB records.
///
/// Serves as a unified validation target for PATCH requests.
pub struct MergedBrandData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
}

impl<'a> MergedBrandData<'a> {
    /// Validates the merged payload state against domain invariants.
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
// SECTION 3: WEB HTML FORM DTOS & VALIDATION
// ============================================================================
//
// ملاحظة هندسية:
// على عكس API PATCH التي تقبل حقول اختيارية (Optional)، تحافط نماذج Web HTML Forms 
// على إعادة إرسال الحقول الكاملة مع كل submit. لذلك تُعرف الحقول كـ String صريحة.
// ============================================================================

// ---------------------------------------------------------------------------
// 3.1 Create Brand (HTML Form Payload)
// ---------------------------------------------------------------------------

/// HTML Form payload for creating a new brand.
#[derive(Debug, Deserialize)]
pub struct CreateBrandForm {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

impl CreateBrandForm {
    /// Validates web form submissions returning localized user-facing error messages.
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
// 3.2 Update Brand (HTML Form Payload & Validation Pipeline)
// ---------------------------------------------------------------------------

/// HTML Form payload for updating an existing brand.
#[derive(Debug, Deserialize)]
pub struct UpdateBrandForm {
    pub name: String,
    pub name_ar: String,
    pub notes: Option<String>,
}

/// Unified validator target for Web Forms.
pub struct MergedBrandFormData<'a> {
    pub name: &'a str,
    pub name_ar: &'a str,
}

impl<'a> MergedBrandFormData<'a> {
    /// Validates submitted form values with user-friendly Arabic validation errors.
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
    /// Delegates form validation to `MergedBrandFormData` to enforce DRY validation patterns.
    pub fn validate(&self) -> Result<(), String> {
        let merged = MergedBrandFormData {
            name: &self.name,
            name_ar: &self.name_ar,
        };
        merged.validate()
    }
}

// ============================================================================
// SECTION 4: ASKAMA TEMPLATES & UI FILTERS
// ============================================================================

/// Main page Askama template rendering brand management dashboard (`brands.html`).
#[derive(Template, WebTemplate)]
#[template(path = "brands.html")]
pub struct BrandsTemplate {
    pub brands: Vec<BrandResponseDto>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub edit_brand: Option<BrandResponseDto>,
    pub current_page: String,
}

/// Partial HTML snippet template for HTMX/Dynamic live brand search.
#[derive(Template, WebTemplate)]
#[template(path = "partials/brand_search_results.html")]
pub struct BrandSearchResultsTemplate {
    pub brands: Vec<BrandResponseDto>,
    pub query: String,
}

/// URL Query parameter extractor for brand live-search requests.
#[derive(Debug, Deserialize)]
pub struct BrandSearchQuery {
    #[serde(default)]
    pub q: String,
}

// ---------------------------------------------------------------------------
// 4.1 Custom Askama Filters
// ---------------------------------------------------------------------------

pub mod filters {
    use askama::Values;

    /// Extract the capitalized initial character of a brand name for UI avatar circles.
    #[askama::filter_fn]
    pub fn first_letter(name: &str, _values: &dyn Values) -> askama::Result<String> {
        Ok(name
            .trim()
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "؟".to_string()))
    }

    /// Generates a deterministic hex color code based on the brand string hash.
    /// Ensures the same brand always gets the exact same avatar background color across renders.
    #[askama::filter_fn]
    pub fn initial_color(name: &str, _values: &dyn Values) -> askama::Result<String> {
        const PALETTE: [&str; 6] = [
            "#0E7C66", "#2563EB", "#D97706", "#7C3AED", "#DB2777", "#0891B2",
        ];
        let sum: u32 = name.bytes().map(|b| b as u32).sum();
        Ok(PALETTE[sum as usize % PALETTE.len()].to_string())
    }
}