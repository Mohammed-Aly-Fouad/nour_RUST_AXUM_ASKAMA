use crate::common::ApiResponse;
use crate::domain::product::dto::{
    ProductResponseDto
};
use crate::state::AppState;
use axum::http::response;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};

/// Configures and returns the sub-router for all product-related JSON API endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_products))
        .route("/{id}", get(get_product))
}

// ==================== 1. List Products ====================

/// Returns all products ordered by ID descending (most recent first).
pub async fn list_products(State(state): State<AppState>) -> impl IntoResponse {
    let products = sqlx::query_as!(
        ProductResponseDto,
        r#"
        SELECT id, name, name_ar,category_id, notes, created_at, updated_at
        FROM products
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await;

    match products {
        Ok(list) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Products retrieved successfully".to_string(),
                data: Some(list)
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<Vec<ProductResponseDto>>::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), "Internal server error occurred".to_string(),
        );
        (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            .into_response()
        }
    }
}

// ==================== 2. Get Product by ID ====================

/// Retrieves a single product by its ID.
pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let product = sqlx::query_as!(
        ProductResponseDto,
        r#"
        SELECT id, name, name_ar, category_id, notes, created_at, updated_at
        FROM products
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await;

    match product {
        Ok(Some(product)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Product retrieved successfully".to_string(),
                data: Some(product),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response = ApiResponse::<ProductResponseDto>::error(
                StatusCode::NOT_FOUND.as_u16(),
                 format!("Product with ID {} not found", id)
                );
                (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<ProductResponseDto>::error(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), "Internal server error occurred".to_string(),
        );
        (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}