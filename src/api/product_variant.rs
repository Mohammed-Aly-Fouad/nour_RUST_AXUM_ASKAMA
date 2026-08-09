use crate::common::ApiResponse;
use crate::domain::product_variant::dto::{
    CreateProductVariantApiDto, ProductVariantResponseDto, UpdateProductVariantApiDto,
};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};

/// Configures and returns the sub-router for all product variant JSON API endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_product_variants))
        .route("/{id}", get(get_product_variant))
        .route("/", post(create_product_variant))
        .route("/{id}", patch(update_product_variant))
        .route("/{id}", delete(delete_product_variant))
}

// ==================== 1. List Product Variants ====================

/// Returns all product variants ordered by ID descending.
pub async fn list_product_variants(State(state): State<AppState>) -> impl IntoResponse {
    let variants = sqlx::query_as!(
        ProductVariantResponseDto,
        r#"
        SELECT 
            id, 
            product_id, 
            brand_id, 
            sku, 
            barcode, 
            name, 
            name_ar, 
            attr, 
            notes, 
            is_active, 
            reorder_threshold, 
            shelf_location, 
            stock_quantity, 
            created_at, 
            updated_at
        FROM product_variants
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await;

    match variants {
        Ok(list) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Product variants retrieved successfully".to_string(),
                data: Some(list),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<Vec<ProductVariantResponseDto>>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Internal server error occurred".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ==================== 2. Get Product Variant by ID ====================

/// Retrieves a single product variant by its ID.
pub async fn get_product_variant(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let variant = sqlx::query_as!(
        ProductVariantResponseDto,
        r#"
        SELECT 
            id, 
            product_id, 
            brand_id, 
            sku, 
            barcode, 
            name, 
            name_ar, 
            attr, 
            notes, 
            is_active, 
            reorder_threshold, 
            shelf_location, 
            stock_quantity, 
            created_at, 
            updated_at
        FROM product_variants
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await;

    match variant {
        Ok(Some(variant)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Product variant retrieved successfully".to_string(),
                data: Some(variant),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response = ApiResponse::<ProductVariantResponseDto>::error(
                StatusCode::NOT_FOUND.as_u16(),
                format!("Product variant with ID {} not found", id),
            );
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<ProductVariantResponseDto>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Internal server error occurred".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ==================== 3. Create Product Variant ====================

/// Creates a new product variant after validating the payload.
pub async fn create_product_variant(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductVariantApiDto>,
) -> impl IntoResponse {
    // 1. Validate payload format via pool
    if let Err((status, message)) = payload.validate(&state.pool).await {
        let response = ApiResponse::<()>::error(status.as_u16(), message);
        return (status, Json(response)).into_response();
    }

    let trimmed_sku = payload.sku.trim();
    let trimmed_barcode: Option<&str> = payload.barcode.as_deref().map(str::trim);
    let trimmed_name = payload.name.trim();
    let trimmed_name_ar = payload.name_ar.trim();
    let trimmed_notes: Option<&str> = payload.notes.as_deref().map(str::trim);
    let trimmed_shelf: Option<&str> = payload.shelf_location.as_deref().map(str::trim);

    // 2. Insert variant into database
    let inserted_variant = sqlx::query_as!(
        ProductVariantResponseDto,
        r#"
        INSERT INTO product_variants (
            product_id, 
            brand_id, 
            sku, 
            barcode, 
            name, 
            name_ar, 
            attr, 
            notes,
            is_active,
            reorder_threshold,
            shelf_location,
            stock_quantity
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, COALESCE($9, true), $10, $11, COALESCE($12, 0))
        RETURNING 
            id, 
            product_id, 
            brand_id, 
            sku, 
            barcode, 
            name, 
            name_ar, 
            attr, 
            notes, 
            is_active, 
            reorder_threshold, 
            shelf_location, 
            stock_quantity, 
            created_at, 
            updated_at
        "#,
        payload.product_id,
        payload.brand_id,
        trimmed_sku,
        trimmed_barcode,
        trimmed_name,
        trimmed_name_ar,
        payload.attr,
        trimmed_notes,
        payload.is_active,
        payload.reorder_threshold,
        trimmed_shelf,
        payload.stock_quantity
    )
    .fetch_one(&state.pool)
    .await;

    // 3. Match database results & constraints
    match inserted_variant {
        Ok(variant) => {
            let response = ApiResponse {
                status: StatusCode::CREATED.as_u16(),
                message: "Product variant created successfully".to_string(),
                data: Some(variant),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                format!("Referenced product (ID {}) or brand does not exist", payload.product_id),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Product variant SKU, barcode, or name already exists".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to insert product variant into database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ==================== 4. Update Product Variant ====================

/// Updates an existing product variant by ID.
pub async fn update_product_variant(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateProductVariantApiDto>,
) -> impl IntoResponse {
    // 1. Fetch existing variant first
    let _existing_variant = match sqlx::query_as!(
        ProductVariantResponseDto,
        r#"
        SELECT 
            id, 
            product_id, 
            brand_id, 
            sku, 
            barcode, 
            name, 
            name_ar, 
            attr, 
            notes, 
            is_active, 
            reorder_threshold, 
            shelf_location, 
            stock_quantity, 
            created_at, 
            updated_at
        FROM product_variants
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(variant)) => variant,
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                format!("Product variant with ID {} not found", id),
            );
            return (StatusCode::NOT_FOUND, Json(response)).into_response();
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to query product variant from database".to_string(),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response();
        }
    };

    // 2. Prepare optional fields cleanly
    let target_brand_id: Option<i64> = payload.brand_id;
    let target_sku: Option<&str> = payload.sku.as_deref().map(str::trim);
    let target_barcode: Option<&str> = payload.barcode.as_deref().map(str::trim);
    let trimmed_name: Option<&str> = payload.name.as_deref().map(str::trim);
    let trimmed_name_ar: Option<&str> = payload.name_ar.as_deref().map(str::trim);
    let target_notes: Option<&str> = payload.notes.as_deref().map(str::trim);
    let target_reorder_threshold: Option<i32> = payload.reorder_threshold;
    let target_shelf: Option<&str> = payload.shelf_location.as_deref().map(str::trim);

    // 3. Execute UPDATE query
    let updated_variant = sqlx::query_as!(
        ProductVariantResponseDto,
        r#"
        UPDATE product_variants
        SET 
            product_id = COALESCE($1, product_id),
            brand_id = CASE WHEN $2::boolean THEN $3 ELSE brand_id END,
            sku = CASE WHEN $4::boolean THEN $5 ELSE sku END,
            barcode = CASE WHEN $6::boolean THEN $7 ELSE barcode END,
            name = COALESCE($8, name),
            name_ar = COALESCE($9, name_ar),
            attr = CASE WHEN $10::boolean THEN $11 ELSE attr END,
            notes = CASE WHEN $12::boolean THEN $13 ELSE notes END,
            is_active = COALESCE($14, is_active),
            reorder_threshold = CASE WHEN $15::boolean THEN $16 ELSE reorder_threshold END,
            shelf_location = CASE WHEN $17::boolean THEN $18 ELSE shelf_location END,
            stock_quantity = COALESCE($19, stock_quantity),
            updated_at = NOW()
        WHERE id = $20
        RETURNING 
            id, 
            product_id, 
            brand_id, 
            sku, 
            barcode, 
            name, 
            name_ar, 
            attr, 
            notes, 
            is_active, 
            reorder_threshold, 
            shelf_location, 
            stock_quantity, 
            created_at, 
            updated_at
        "#,
        payload.product_id,
        payload.brand_id.is_some(),
        target_brand_id,
        payload.sku.is_some(),
        target_sku,
        payload.barcode.is_some(),
        target_barcode,
        trimmed_name,
        trimmed_name_ar,
        payload.attr.is_some(),
        payload.attr,
        payload.notes.is_some(),
        target_notes,
        payload.is_active,
        payload.reorder_threshold.is_some(),
        target_reorder_threshold,
        payload.shelf_location.is_some(),
        target_shelf,
        payload.stock_quantity,
        id
    )
    .fetch_one(&state.pool)
    .await;

    // 4. Handle SQL execution errors
    match updated_variant {
        Ok(variant) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Product variant updated successfully".to_string(),
                data: Some(variant),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Referenced product or brand does not exist".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Product variant SKU, barcode, or name already exists".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to update product variant in database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ==================== 5. Delete Product Variant By ID ====================

/// Deletes a product variant by its ID.
pub async fn delete_product_variant(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let deleted_variant = sqlx::query_as!(
        ProductVariantResponseDto,
        r#"
        DELETE FROM product_variants
        WHERE id = $1
        RETURNING 
            id, 
            product_id, 
            brand_id, 
            sku, 
            barcode, 
            name, 
            name_ar, 
            attr, 
            notes, 
            is_active, 
            reorder_threshold, 
            shelf_location, 
            stock_quantity, 
            created_at, 
            updated_at
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await;

    match deleted_variant {
        Ok(Some(variant)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Product variant deleted successfully".to_string(),
                data: Some(variant),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                format!("Product variant with ID {} not found", id),
            );
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            let response = ApiResponse::<()>::error(
                StatusCode::CONFLICT.as_u16(),
                format!(
                    "Cannot delete product variant ID {} because it is referenced in active inventory or transactions",
                    id
                ),
            );
            (StatusCode::CONFLICT, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to delete product variant from database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}