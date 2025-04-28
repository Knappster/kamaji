use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::state::AppState;

pub mod api;

pub fn routes() -> Router<AppState> {
    let static_routes =
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    Router::new()
        .nest("/api", api::routes())
        .fallback_service(static_routes)
}
