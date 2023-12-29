use axum::{routing::get, Router};
use std::{env, path::PathBuf};
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;

use crate::AppState;

pub fn get_routes(state: AppState) -> Router {
    let user_routes = Router::new().route("/", get(|| async { "Users route!" }));

    let api_routes = Router::new().nest("/users", user_routes);

    let assets_dir = PathBuf::from(env::var("ASSETS_PATH").expect("Assets path env var missing!"));
    let static_routes = Router::new().nest_service("/", ServeDir::new(assets_dir));

    Router::new()
        .nest("/api", api_routes)
        .merge(static_routes)
        .with_state(state)
        .layer(CompressionLayer::new())
}
