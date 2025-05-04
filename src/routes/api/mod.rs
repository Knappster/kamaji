use axum::Router;

use crate::state::State;

pub mod users;

pub fn get_routes() -> Router<State> {
    Router::new().nest("/users", users::get_routes())
}
