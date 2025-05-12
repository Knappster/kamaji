use axum::Router;

use crate::state::State;

pub mod users;

pub fn routes() -> Router<State> {
    Router::new().nest("/users", users::routes())
}
