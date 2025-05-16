use axum::routing::get;
use axum::Router;

use crate::services::users::*;
use crate::state::State;

pub fn routes() -> Router<State> {
    Router::new()
        .route("/{id}", get(get_user))
        .route("/test_one", get(test_one))
        .route("/test_two", get(test_two))
}
