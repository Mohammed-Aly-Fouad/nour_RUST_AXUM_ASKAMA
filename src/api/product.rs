use crate::common::ApiResponse;
use crate::domain::product::dto::{
    CreateProductDto, ProductResponseDto, UpdateProductDto
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
        .route("/{id}", patch(update_product))
        .route("/{id}", delete(delete_product))
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
    Path(id): Path<i64>,
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

// ==================== 3. Create Product ====================

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
// ==================== 4. Update Product ====================

/// Updates an existing product by ID, skipping DB write if no actual changes occurred.
/// 
pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateProductDto>,
) -> impl IntoResponse {
    // 1. Validate payload format (in-memory)
    if let Err((status, message)) = payload.validate() {
        let response = ApiResponse::<()>::error(status.as_u16(), message);
        return (status, Json(response)).into_response();
    }

    // 2. Fetch existing product to check existence and current values
    let existing_product = match sqlx::query_as!(
        ProductResponseDto,
        r#"
        SELECT id, name, name_ar, category_id, notes, created_at, updated_at
        FROM products
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(product)) => product,
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                format!("Product with ID {} not found", id),
            );
            return (StatusCode::NOT_FOUND, Json(response)).into_response();
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to query product from database".to_string(),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response();
        }
    };

    // 3. Compare payload fields against existing database values
    let trimmed_name = payload.name.as_ref().map(|n| n.trim());
    let trimmed_name_ar = payload.name_ar.as_ref().map(|n| n.trim());
    
    // Convert payload.notes into final target value: Option<&str>
    let target_notes = payload.notes.as_ref().and_then(|n| n.as_deref().map(|s| s.trim()));

    let is_name_changed = trimmed_name.map_or(false, |name| name != existing_product.name);
    let is_name_ar_changed = trimmed_name_ar.map_or(false, |name_ar| name_ar != existing_product.name_ar);
    let is_category_changed = payload.category_id.map_or(false, |cat_id| cat_id != existing_product.category_id);
    let is_notes_changed = match &payload.notes {
        Some(_) => target_notes != existing_product.notes.as_deref(),
        None => false, // Field was omitted entirely, so no change requested
    };

    // 4. Early return if nothing actually changed
    if !is_name_changed && !is_name_ar_changed && !is_category_changed && !is_notes_changed {
        let response = ApiResponse {
            status: StatusCode::OK.as_u16(),
            message: "No changes detected".to_string(),
            data: Some(existing_product),
        };
        return (StatusCode::OK, Json(response)).into_response();
    }

    // 5. Execute UPDATE only when real changes exist
    let updated_product = sqlx::query_as!(
        ProductResponseDto,
        r#"
        UPDATE products
        SET 
            name = COALESCE($1, name),
            name_ar = COALESCE($2, name_ar),
            category_id = COALESCE($3, category_id),
            notes = CASE 
                WHEN $4::boolean THEN $5 
                ELSE notes 
            END,
            updated_at = NOW()
        WHERE id = $6
        RETURNING id, name, name_ar, category_id, notes, created_at, updated_at
        "#,
        trimmed_name,
        trimmed_name_ar,
        payload.category_id,
        payload.notes.is_some(),
        target_notes,
        id
    )
    .fetch_one(&state.pool)
    .await;

    // 6. Match database results & foreign key / constraint errors
    match updated_product {
        Ok(product) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Product updated successfully".to_string(),
                data: Some(product),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        // PostgreSQL Code 23503 = Foreign Key Violation (Invalid Category ID)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            let category_id = payload.category_id.unwrap_or_default();
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                format!("Category with ID {} does not exist", category_id),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        // PostgreSQL Code 23505 = Unique Constraint Violation
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
                "Failed to update product in database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}


// ==================== 5. Delete Product By ID ====================

/// Deletes a product by its ID.
pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // 1. Execute DELETE query and return basic info of the deleted record
    let deleted_product = sqlx::query_as!(
        ProductResponseDto,
        r#"
        DELETE FROM products
        WHERE id = $1
        RETURNING id, name, name_ar, category_id, notes, created_at, updated_at
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await;

    // 2. Match database results & handle potential relation errors
    match deleted_product {
        // Successfully deleted
        Ok(Some(product)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Product deleted successfully".to_string(),
                data: Some(product),
            };
            (StatusCode::OK, Json(response)).into_response()
        }

        // Product ID does not exist
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                format!("Product with ID {} not found", id),
            );
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }

        // PostgreSQL Code 23503 = Foreign Key Violation
        // (e.g., Product is referenced in sales, order_items, inventory tables, etc.)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            let response = ApiResponse::<()>::error(
                StatusCode::CONFLICT.as_u16(),
                format!(
                    "Cannot delete product ID {} because it is referenced in other records (e.g., sales or inventory)",
                    id
                ),
            );
            (StatusCode::CONFLICT, Json(response)).into_response()
        }

        // General internal database error
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to delete product from database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}