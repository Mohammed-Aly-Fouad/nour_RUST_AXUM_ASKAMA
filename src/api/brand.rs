use crate::common::ApiResponse;
use crate::domain::brand::dto::{
    BrandResponseDto, CreateBrandDto, MergedBrandData, UpdateBrandDto,
};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};

// ============================================================================
// ROUTER CONFIGURATION
// ============================================================================

/// Configures and returns the sub-router for all brand-related JSON API endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        // GET /api/brands -> List all brands
        .route("/", get(list_brands))
        // GET /api/brands/{id} -> Get a single brand by ID
        .route("/{id}", get(get_brand))
        // POST /api/brands -> Create a new brand
        .route("/", post(create_brand))
        // PATCH /api/brands/{id} -> Partial update (supports optional payload fields)
        .route("/{id}", patch(update_brand))
        // DELETE /api/brands/{id} -> Delete a brand by ID
        .route("/{id}", delete(delete_brand))
}

// ============================================================================
// HANDLERS: READ OPERATIONS (GET)
// ============================================================================

// ---------------------------------------------------------------------------
// 1. List Brands
// ---------------------------------------------------------------------------

/// Retrieves all brands sorted in descending order by ID (newest first).
pub async fn list_brands(State(state): State<AppState>) -> impl IntoResponse {
    let brands = sqlx::query_as!(
        BrandResponseDto,
        r#"
        SELECT id, name, name_ar, notes, created_at, updated_at
        FROM brands
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await;

    match brands {
        Ok(list) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Brands retrieved successfully".to_string(),
                data: Some(list),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Failed to fetch brands list: {:?}", err);
            let response = ApiResponse::<Vec<BrandResponseDto>>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Internal server error occurred".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Get Brand by ID
// ---------------------------------------------------------------------------

/// Retrieves a single brand entity by its unique identifier.
pub async fn get_brand(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
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
    .await;

    match brand {
        Ok(Some(b)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Brand retrieved successfully".to_string(),
                data: Some(b),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response = ApiResponse::<BrandResponseDto>::error(
                StatusCode::NOT_FOUND.as_u16(),
                format!("Brand with ID {} not found", id),
            );
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Failed to fetch brand with ID {}: {:?}", id, err);
            let response = ApiResponse::<BrandResponseDto>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Internal server error occurred".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ============================================================================
// HANDLERS: WRITE OPERATIONS (POST, PATCH, DELETE)
// ============================================================================

// ---------------------------------------------------------------------------
// 3. Create Brand
// ---------------------------------------------------------------------------

/// Creates a new brand entity after validating the payload.
pub async fn create_brand(
    State(state): State<AppState>,
    Json(payload): Json<CreateBrandDto>,
) -> impl IntoResponse {
    // 1. Validation check
    if let Err((status, message)) = payload.validate() {
        let response = ApiResponse::<()>::error(status.as_u16(), message);
        return (status, Json(response)).into_response();
    }

    let trimmed_name = payload.name.trim();
    let trimmed_name_ar = payload.name_ar.trim();

    // 2. Database Insert Operation
    let inserted_brand = sqlx::query_as!(
        BrandResponseDto,
        r#"
        INSERT INTO brands (name, name_ar, notes)
        VALUES ($1, $2, $3)
        RETURNING id, name, name_ar, notes, created_at, updated_at
        "#,
        trimmed_name,
        trimmed_name_ar,
        payload.notes
    )
    .fetch_one(&state.pool)
    .await;

    // 3. Response handling
    match inserted_brand {
        Ok(brand) => {
            let response = ApiResponse {
                status: StatusCode::CREATED.as_u16(),
                message: "Brand created successfully".to_string(),
                data: Some(brand),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        // Error 23505: Unique Constraint Violation (اسم مكرر)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Brand name already exists".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Failed to create brand: {:?}", err);
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to insert brand into database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ---------------------------------------------------------------------------
// 4. Update Brand (PATCH)
// ---------------------------------------------------------------------------

/// Updates specific fields of an existing brand. Unspecified fields (`None`) retain current values.
pub async fn update_brand(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateBrandDto>,
) -> impl IntoResponse {
    // 1. Fetch current record to verify existence
    let old_brand = match sqlx::query_as!(
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
    {
        Ok(Some(b)) => b,
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                "Brand not found".to_string(),
            );
            return (StatusCode::NOT_FOUND, Json(response)).into_response();
        }
        Err(err) => {
            tracing::error!("Failed to fetch brand before updating (ID {}): {:?}", id, err);
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Database error while fetching brand".to_string(),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response();
        }
    };

    // 2. Apply PATCH semantics (Merge new inputs with existing entity data)
    let final_name = payload.name.as_deref().unwrap_or(&old_brand.name);
    let final_name_ar = payload.name_ar.as_deref().unwrap_or(&old_brand.name_ar);
    let final_notes = payload.notes.clone().or_else(|| old_brand.notes.clone());

    // 3. Early return if payload causes no state change
    if final_name == old_brand.name
        && final_name_ar == old_brand.name_ar
        && final_notes == old_brand.notes
    {
        let response = ApiResponse {
            status: StatusCode::OK.as_u16(),
            message: "No changes detected, brand remains unchanged".to_string(),
            data: Some(old_brand),
        };
        return (StatusCode::OK, Json(response)).into_response();
    }

    // 4. Validate merged values
    let merged = MergedBrandData {
        name: final_name,
        name_ar: final_name_ar,
    };

    if let Err((status, message)) = merged.validate() {
        let response = ApiResponse::<()>::error(status.as_u16(), message);
        return (status, Json(response)).into_response();
    }

    let trimmed_name = final_name.trim();
    let trimmed_name_ar = final_name_ar.trim();

    // 5. Save changes to DB
    let updated_brand = sqlx::query_as!(
        BrandResponseDto,
        r#"
        UPDATE brands
        SET name = $1, name_ar = $2, notes = $3, updated_at = NOW()
        WHERE id = $4
        RETURNING id, name, name_ar, notes, created_at, updated_at
        "#,
        trimmed_name,
        trimmed_name_ar,
        final_notes,
        id
    )
    .fetch_one(&state.pool)
    .await;

    match updated_brand {
        Ok(brand) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Brand updated successfully".to_string(),
                data: Some(brand),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        // Error 23505: Unique Constraint Violation (اسم مكرر)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Brand name already exists".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Failed to update brand ID {}: {:?}", id, err);
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to update brand in database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ---------------------------------------------------------------------------
// 5. Delete Brand
// ---------------------------------------------------------------------------

/// Deletes a brand entity by ID. Handles foreign key constraint errors gracefully.
pub async fn delete_brand(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        DELETE FROM brands
        WHERE id = $1
        RETURNING id
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await;

    match result {
        // Successful deletion
        Ok(Some(_)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Brand deleted successfully".to_string(),
                data: None::<()>,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        // Entity not found
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                "Brand not found".to_string(),
            );
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        // Error 23503: Foreign Key Violation (مرتبط بسجلات أخرى مثل المنتجات)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Cannot delete this brand because it has related records linked to it. Please remove or reassign them first.".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(err) => {
            tracing::error!("Failed to delete brand ID {}: {:?}", id, err);
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to delete brand from database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}