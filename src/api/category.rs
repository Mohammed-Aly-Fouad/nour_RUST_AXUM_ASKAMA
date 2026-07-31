use crate::domain::category::dto::CategoryResponseDto;
use crate::state::AppState;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};

/// Configures and returns the sub-router for all category-related JSON API endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        // GET /api/categories -> List all categories
        .route("/", get(list_categories))
        .route("/{id}", get(get_category))
}

/// 1- Retrieve a list of all existing categories ordered by ID descending via JSON API
pub async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoryResponseDto>>, StatusCode> {
    let categories = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at 
        FROM categories 
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(categories))
}


/// 2. Retrieve a single brand by its unique ID via JSON API
pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<CategoryResponseDto>, StatusCode> {
    let category = sqlx::query_as!(
        CategoryResponseDto,
        r#"
       SELECT id, name, name_ar, parent_id, notes, created_at, updated_at 
        FROM categories 
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match category {
        Some(b) => Ok(Json(b)),
        None => Err(StatusCode::NOT_FOUND),
    }
}