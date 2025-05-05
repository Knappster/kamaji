use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use crate::state::State;

mod api;
//mod oauth;

pub fn get_routes() -> Router<State> {
    let static_routes =
        ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    Router::new()
        .nest("/api", api::get_routes())
        //.merge(oauth::get_routes())
        .fallback_service(static_routes)
}
