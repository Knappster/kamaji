use axum::{routing::get, Router};
use std::{env, path::PathBuf};
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tracing::{debug, instrument};

use crate::state::AppState;

#[instrument]
pub fn get_routes(state: AppState) -> Router {
    let user_routes = Router::new().route("/", get(|| async { "Users route!" }));

    let api_routes = Router::new().nest("/users", user_routes);

    let assets_dir = PathBuf::from(env::var("ASSETS_PATH").unwrap_or_else(|e| {
        debug!("ASSETS_PATH: {}", e);
        "/public".to_string()
    }));
    debug!("assets_dir={}", assets_dir.display().to_string());

    let static_routes = Router::new().nest_service("/", ServeDir::new(assets_dir));

    Router::new()
        .nest("/api", api_routes)
        .merge(static_routes)
        .with_state(state)
        .layer(CompressionLayer::new())
}
