mod api;
mod assets;
//mod oauth;

use axum::Router;

use crate::config::Config;
use crate::state::State;

pub fn routes(config: Config) -> Router<State> {
    Router::new()
        .nest("/api", api::routes())
        //.merge(oauth::get_routes())
        .fallback_service(assets::service(config))
}
