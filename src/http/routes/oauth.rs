use axum::routing::get;
use axum::Router;

use crate::state::StateType;

pub fn routes() -> Router<StateType> {
    Router::new()
        .route("/auth/discord", get(discord_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/protected", get(protected))
        .route("/logout", get(logout))
}
