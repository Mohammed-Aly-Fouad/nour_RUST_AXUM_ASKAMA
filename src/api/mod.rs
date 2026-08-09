pub mod brand;
pub mod category;
pub mod product;
pub mod product_variant;

use axum::Router;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/brands", brand::router())
        .nest("/categories", category::router())
        .nest("/products", product::router())
        
}