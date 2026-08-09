use crate::common::ApiResponse;
use crate::domain::category::dto::{
    CategoryResponseDto, CategoryTreeDto, CreateCategoryApiDto, MergedCategoryData,
    UpdateCategoryApiDto,
};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};

/// Configures and returns the sub-router for all category-related JSON API endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        // GET /api/categories -> List all categories as a nested tree
        .route("/", get(list_categories_tree))
        // GET /api/categories/{id} -> Get a single category by ID
        .route("/{id}", get(get_category))
        // POST /api/categories -> Create a new category
        .route("/", post(create_category))
        // PATCH /api/categories/{id} -> Partial update (matches the optional-fields DTO)
        .route("/{id}", patch(update_category))
        // DELETE /api/categories/{id} -> Delete a category
        .route("/{id}", delete(delete_category))
}

// ==================== 1. List Categories (Tree) ====================

/// Returns all categories as a nested tree structure (parent -> children).
pub async fn list_categories_tree(State(state): State<AppState>) -> impl IntoResponse {
    // 1. Fetch all categories as a flat list
    let categories = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
        FROM categories
        ORDER BY id ASC
        "#
    )
    .fetch_all(&state.pool)
    .await;

    match categories {
        Ok(flat_cats) => {
            // 2. Convert the flat list into a nested tree
            let tree = CategoryTreeDto::build_tree(flat_cats);

            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Categories tree retrieved successfully".to_string(),
                data: Some(tree),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<Vec<CategoryTreeDto>>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Internal server error occurred".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ==================== 2. Get Category by ID ====================

/// Retrieves a single category by its ID.
pub async fn get_category(
    State(state): State<AppState>,
    Path(id_str): Path<String>,
) -> impl IntoResponse {
    // 1. Parse the ID from String to i32 (gives a clear, unified error message on failure)
    let id = match id_str.parse::<i64>() {
        Ok(parsed_id) => parsed_id,
        Err(_) => {
            let response = ApiResponse::<CategoryResponseDto>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                format!("Invalid ID format: '{}'. Expected a valid number.", id_str),
            );
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    // 2. Fetch the category from the database
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
    .await;

    match category {
        Ok(Some(cat)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Category retrieved successfully".to_string(),
                data: Some(cat),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => {
            let response = ApiResponse::<CategoryResponseDto>::error(
                StatusCode::NOT_FOUND.as_u16(),
                format!("Category with ID {} not found", id),
            );
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<CategoryResponseDto>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Internal server error occurred".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ==================== 3. Create Category ====================

/// Creates a new category after validating the incoming payload.
pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryApiDto>,
) -> impl IntoResponse {
    // 1. Validate the incoming payload (name length, parent existence, etc.)
    if let Err((status, message)) = payload.validate(&state.pool).await {
        let response = ApiResponse::<()>::error(status.as_u16(), message);
        return (status, Json(response)).into_response();
    }

    let trimmed_name = payload.name.trim();
    let trimmed_name_ar = payload.name_ar.trim();

    // 2. Insert the new category into the database
    let inserted_category = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        INSERT INTO categories (name, name_ar, parent_id, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, name_ar, parent_id, notes, created_at, updated_at
        "#,
        trimmed_name,
        trimmed_name_ar,
        payload.parent_id,
        payload.notes
    )
    .fetch_one(&state.pool)
    .await;

    match inserted_category {
        Ok(cat) => {
            let response = ApiResponse {
                status: StatusCode::CREATED.as_u16(),
                message: "Category created successfully".to_string(),
                data: Some(cat),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Category name already exists".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to insert category into database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ==================== 4. Update Category (Partial - PATCH) ====================

/// Partially updates a category. Only the fields present in the payload are changed;
/// any field left out (`None`) keeps its existing value.
pub async fn update_category(
    State(state): State<AppState>,
    Path(category_id): Path<i64>,
    Json(payload): Json<UpdateCategoryApiDto>,
) -> impl IntoResponse {
    // 1. Fetch the existing category to confirm it exists
    let old_category = match sqlx::query_as!(
        CategoryResponseDto,
        r#"
        SELECT id, name, name_ar, parent_id, notes, created_at, updated_at
        FROM categories
        WHERE id = $1
        "#,
        category_id
    )
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(cat)) => cat,
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                "Category not found".to_string(),
            );
            return (StatusCode::NOT_FOUND, Json(response)).into_response();
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Database error while fetching category".to_string(),
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response();
        }
    };

    // 2. Merge incoming values with the existing ones (PATCH semantics)
    let final_name = payload.name.as_deref().unwrap_or(&old_category.name);
    let final_name_ar = payload.name_ar.as_deref().unwrap_or(&old_category.name_ar);
    let final_parent_id = payload.parent_id.or(old_category.parent_id);
    let final_notes = payload.notes.clone().or_else(|| old_category.notes.clone());

    // 3. Early exit if nothing actually changed
    if final_name == old_category.name
        && final_name_ar == old_category.name_ar
        && final_parent_id == old_category.parent_id
        && final_notes == old_category.notes
    {
        let response = ApiResponse {
            status: StatusCode::OK.as_u16(),
            message: "No changes detected, category remains unchanged".to_string(),
            data: Some(old_category),
        };
        return (StatusCode::OK, Json(response)).into_response();
    }

    // 4. Validate the merged (final) values
    //    old_category.parent_id is passed so validate() can tell whether this category
    //    was previously a top-level (parent) category before the update
    let merged = MergedCategoryData {
        name: final_name,
        name_ar: final_name_ar,
        parent_id: final_parent_id,
    };

    if let Err((status, message)) = merged
        .validate(category_id, old_category.parent_id, &state.pool)
        .await
    {
        let response = ApiResponse::<()>::error(status.as_u16(), message);
        return (status, Json(response)).into_response();
    }

    let trimmed_name = final_name.trim();
    let trimmed_name_ar = final_name_ar.trim();

    // 5. Persist the update
    let updated_category = sqlx::query_as!(
        CategoryResponseDto,
        r#"
        UPDATE categories
        SET name = $1, name_ar = $2, parent_id = $3, notes = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING id, name, name_ar, parent_id, notes, created_at, updated_at
        "#,
        trimmed_name,
        trimmed_name_ar,
        final_parent_id,
        final_notes,
        category_id
    )
    .fetch_one(&state.pool)
    .await;

    match updated_category {
        Ok(cat) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Category updated successfully".to_string(),
                data: Some(cat),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Category name already exists".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to update category in database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}

// ==================== 5. Delete Category ====================

/// Deletes a category by ID.
/// Fails with a clear message if the category still has sub-categories
/// (or other related records) pointing to it via a foreign key.
pub async fn delete_category(
    State(state): State<AppState>,
    Path(category_id): Path<i64>,
) -> impl IntoResponse {
    // Attempt the delete directly; RETURNING tells us whether a row actually existed
    let result = sqlx::query!(
        r#"
        DELETE FROM categories
        WHERE id = $1
        RETURNING id
        "#,
        category_id
    )
    .fetch_optional(&state.pool)
    .await;

    match result {
        // Category found and deleted successfully
        Ok(Some(_)) => {
            let response = ApiResponse {
                status: StatusCode::OK.as_u16(),
                message: "Category deleted successfully".to_string(),
                data: None::<()>,
            };
            (StatusCode::OK, Json(response)).into_response()
        }
        // No category with this ID
        Ok(None) => {
            let response = ApiResponse::<()>::error(
                StatusCode::NOT_FOUND.as_u16(),
                "Category not found".to_string(),
            );
            (StatusCode::NOT_FOUND, Json(response)).into_response()
        }
        // 23503: foreign key violation - happens when the category still has
        // sub-categories (or other related records) pointing to it
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23503") => {
            let response = ApiResponse::<()>::error(
                StatusCode::BAD_REQUEST.as_u16(),
                "Cannot delete this category because it has sub-categories (or related records) linked to it. Please remove or reassign them first.".to_string(),
            );
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
        // Any other database error
        Err(_) => {
            let response = ApiResponse::<()>::error(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "Failed to delete category from database".to_string(),
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response)).into_response()
        }
    }
}
