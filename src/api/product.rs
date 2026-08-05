use crate::common::ApiResponse;
use crate::domain::product::dto::{
    CreateProductDto, ProductResponseDto
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
        .route("/", post(create_product))
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

// ==================== 3. Create Brand ====================

/// Creates a new product after validating the incoming payload.
pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductDto>,
) -> impl IntoResponse {
    // 1. Validate payload format (in-memory)
    if let Err((status, message)) = payload.validate() {
        let response = ApiResponse::<()>::error(status.as_u16(), message);
        return (status, Json(response)).into_response();
    }

    let trimmed_name = payload.name.trim();
    let trimmed_name_ar = payload.name_ar.trim();
    let trimmed_notes = payload.notes.as_ref().map(|n| n.trim());

    // 2. Insert product into database
    let inserted_product = sqlx::query_as!(
        ProductResponseDto,
        r#"
        INSERT INTO products (name, name_ar, category_id, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, name_ar, category_id, notes, created_at, updated_at
        "#,
        trimmed_name,
        trimmed_name_ar,
        payload.category_id,
        trimmed_notes
    )
    .fetch_one(&state.pool)
    .await;

    // 3. Match database results & foreign key violations
    match inserted_product {
        Ok(product) => {
            let response = ApiResponse {
                status: StatusCode::CREATED.as_u16(),
                message: "Product created successfully".to_string(),
                data: Some(product),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        // PostgreSQL Code 23503 = Foreign Key Constraint Violation (Invalid Category ID)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                format!("Category with ID {} does not exist", payload.category_id),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        // PostgreSQL Code 23505 = Unique Violation (If product name has a UNIQUE constraint)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Product name already exists".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to insert product into database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}