use axum::routing::get;
use axum::Router;

use crate::handlers::users;
use crate::state::State;

pub fn routes() -> Router<State> {
    Router::new()
        .route("/{id}", get(users::get_user))
        .route("/test_one", get(users::test_one))
        .route("/test_two", get(users::test_two))
}
