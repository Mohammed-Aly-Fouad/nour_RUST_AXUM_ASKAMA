use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};



use crate::state::AppState;
use crate::domain::brand::dto::{
    BrandResponseDto, CreateBrandForm, UpdateBrandForm, BrandsTemplate,
};

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

/// 1. Render the main HTML page containing the brand list and creation form
pub async fn render_brands_page(State(state): State<AppState>) -> BrandsTemplate {
    let brands = sqlx::query_as!(
        BrandResponseDto,
        r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands ORDER BY id DESC"#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    BrandsTemplate {
        brands,
        error_message: None,
        success_message: None,
        edit_brand: None,
    }
}

/// 2. Handle web form submission for creating a new brand with manual validation
pub async fn create_brand_web(
    State(state): State<AppState>,
    Form(form): Form<CreateBrandForm>,
) -> axum::response::Response {
    // Validate the incoming form data
    if let Err(err_msg) = form.validate() {
        let brands = sqlx::query_as!(
            BrandResponseDto,
            r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands ORDER BY id DESC"#
        )
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();

        // Return template directly as IntoResponse
        return BrandsTemplate {
            brands,
            error_message: Some(err_msg),
            success_message: None,
            edit_brand: None,
        }
        .into_response();
    }

    // Insert into database if validation succeeds
    let _ = sqlx::query!(
        r#"INSERT INTO brands (name, name_ar, notes) VALUES ($1, $2, $3)"#,
        form.name,
        form.name_ar,
        form.notes
    )
    .execute(&state.pool)
    .await;

    // Redirect back to the main brands page
    Redirect::to("/brands").into_response()
}

/// 3. Render the page with a specific brand pre-loaded in the edit form
pub async fn edit_brand_page(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> BrandsTemplate {
    let brands = sqlx::query_as!(
        BrandResponseDto,
        r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands ORDER BY id DESC"#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let edit_brand = sqlx::query_as!(
        BrandResponseDto,
        r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap_or(None);

    BrandsTemplate {
        brands,
        error_message: None,
        success_message: None,
        edit_brand,
    }
}

/// 4. Handle web form submission for updating an existing brand with validation
pub async fn update_brand_web(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<UpdateBrandForm>,
) -> axum::response::Response {
    if let Err(err_msg) = form.validate() {
        let brands = sqlx::query_as!(
            BrandResponseDto,
            r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands ORDER BY id DESC"#
        )
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();

        let edit_brand = sqlx::query_as!(
            BrandResponseDto,
            r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands WHERE id = $1"#,
            id
        )
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(None);

        return BrandsTemplate {
            brands,
            error_message: Some(err_msg),
            success_message: None,
            edit_brand,
        }
        .into_response();
    }

    let _ = sqlx::query!(
        r#"UPDATE brands SET name = $1, name_ar = $2, notes = $3, updated_at = now() WHERE id = $4"#,
        form.name,
        form.name_ar,
        form.notes,
        id
    )
    .execute(&state.pool)
    .await;

    Redirect::to("/brands").into_response()
}

/// 5. Handle brand deletion from the web interface
pub async fn delete_brand_web(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> axum::response::Response {
    let _ = sqlx::query!(r#"DELETE FROM brands WHERE id = $1"#, id)
        .execute(&state.pool)
        .await;

    Redirect::to("/brands").into_response()
}