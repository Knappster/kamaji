mod api;
mod assets;
//mod auth;

use axum::Router;
use std::sync::Arc;

use crate::config::Config;
use crate::state::State;

pub fn routes(config: Arc<Config>) -> Router<State> {
    Router::new()
        .nest("/api", api::routes())
        //.merge(auth::routes())
        .fallback_service(assets::service(config))
}
