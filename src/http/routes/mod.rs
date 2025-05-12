use axum::Router;

use crate::config::Config;
use crate::state::State;

mod api;
mod assets;
//mod oauth;

pub fn routes(config: Config) -> Router<State> {
    Router::new()
        .nest("/api", api::routes())
        //.merge(oauth::get_routes())
        .fallback_service(assets::service(config))
}
