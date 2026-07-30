// use axum::{
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     Json,
// };
// use serde_json::json;
// use validator::ValidationErrors;

// #[derive(Debug)]
// #[allow(dead_code)]

// pub enum AppError {
//     NotFound(String),
//     BadRequest(String),
//     ValidationError(ValidationErrors),
//     DatabaseError(sqlx::Error),
// }

// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             AppError::NotFound(msg) => (StatusCode::NOT_FOUND, json!({ "error": msg })),
//             AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, json!({ "error": msg })),
//             AppError::ValidationError(errs) => (
//                 StatusCode::BAD_REQUEST,
//                 json!({ "error": "Validation failed", "details": errs }),
//             ),
//             AppError::DatabaseError(e) => {
//                 eprintln!("Database error: {:?}", e);
//                 (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     json!({ "error": "Internal server error" }),
//                 )
//             }
//         };

//         (status, Json(error_message)).into_response()
//     }
// }

// impl From<sqlx::Error> for AppError {
//     fn from(err: sqlx::Error) -> Self {
//         match err {
//             sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
//             _ => AppError::DatabaseError(err),
//         }
//     }
// }