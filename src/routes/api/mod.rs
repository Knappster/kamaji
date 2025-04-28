use axum::Router;

use crate::state::AppState;

pub mod users;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/users", users::routes())
}
