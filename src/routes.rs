use crate::models::Config;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use axum_extra::routing::SpaRouter;
use diesel::prelude::*;

use crate::AppState;

async fn get_client_id(State(state): State<AppState>) -> Result<String, StatusCode> {
    use crate::schema::config::dsl::*;

    let result = state
        .database
        .query(|conn| {
            config
                .filter(name.eq("client_id"))
                .get_result::<Config>(conn)
        })
        .await;

    match result {
        Ok(result) => Ok(result.value),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn get_router(state: AppState) -> Router {
    let user_routes = Router::new()
        .route("/:id", get(|| async {}))
        .route("/get_config", get(get_client_id));

    let api_routes = Router::new().nest("/users", user_routes);

    let static_routes =
        Router::new().merge(SpaRouter::new("/assets", "assets").index_file("index.html"));

    Router::new()
        .nest("/api", api_routes)
        .merge(static_routes)
        .with_state(state)
}
