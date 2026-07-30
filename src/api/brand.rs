use crate::domain::brand::dto::{BrandResponseDto, CreateBrandDto, UpdateBrandDto};
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};

/// Configures and returns the sub-router for all brand-related JSON API endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        // POST /api/brands -> Create a brand | GET /api/brands -> List all brands
        .route("/", post(create_brand).get(list_brands))
        // GET/PUT/DELETE /api/brands/{id} -> Operations on a specific brand by ID
        .route(
            "/{id}",
            get(get_brand).put(update_brand).delete(delete_brand),
        )
}

/// 1. Create a new brand in the database via JSON payload
pub async fn create_brand(
    State(state): State<AppState>,
    Json(payload): Json<CreateBrandDto>,
) -> Result<(StatusCode, Json<BrandResponseDto>), (StatusCode, String)> {
    let brand = sqlx::query_as!(
        BrandResponseDto,
        r#"
        INSERT INTO brands (name, name_ar, notes)
        VALUES ($1, $2, $3)
        RETURNING id, name, name_ar, notes, created_at, updated_at
        "#,
        payload.name,
        payload.name_ar,
        payload.notes
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, Json(brand)))
}

/// 2. Retrieve a list of all existing brands ordered by ID descending via JSON API
pub async fn list_brands(
    State(state): State<AppState>,
) -> Result<Json<Vec<BrandResponseDto>>, StatusCode> {
    let brands = sqlx::query_as!(
        BrandResponseDto,
        r#"
        SELECT id, name, name_ar, notes, created_at, updated_at 
        FROM brands 
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(brands))
}

/// 3. Retrieve a single brand by its unique ID via JSON API
pub async fn get_brand(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<BrandResponseDto>, StatusCode> {
    let brand = sqlx::query_as!(
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match brand {
        Some(b) => Ok(Json(b)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// 4. Update an existing brand (supports partial updates) via JSON API
pub async fn update_brand(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateBrandDto>,
) -> Result<Json<BrandResponseDto>, StatusCode> {
    let existing = sqlx::query_as!(
        BrandResponseDto,
        r#"SELECT id, name, name_ar, notes, created_at, updated_at FROM brands WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let current = match existing {
        Some(b) => b,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let name: String = payload.name.unwrap_or(current.name);
let name_ar: String = payload.name_ar.unwrap_or(current.name_ar);
    let notes: Option<String> = payload.notes.or(current.notes);

    let updated_brand: BrandResponseDto = sqlx::query_as!(
        BrandResponseDto,
        r#"
        UPDATE brands 
        SET name = $1, name_ar = $2, notes = $3, updated_at = now()
        WHERE id = $4
        RETURNING id, name, name_ar, notes, created_at, updated_at
        "#,
        name,
        name_ar,
        notes,
        id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_brand))
}

/// 5. Delete a brand by its unique ID via JSON API
pub async fn delete_brand(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(r#"DELETE FROM brands WHERE id = $1"#, id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
