pub mod users;

use axum::Router;

use crate::state::State;

pub fn routes() -> Router<State> {
    Router::new().nest("/users", users::routes())
}
