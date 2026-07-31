pub mod brand;
pub mod category;
use axum::Router;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/brands", brand::router())
        .nest("/categories", category::router())
        
}