use crate::models::Config;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use axum_extra::routing::SpaRouter;
use diesel::prelude::*;

use crate::AppState;

async fn get_client_id(State(state): State<AppState>) -> Result<String, StatusCode> {
    use crate::schema::config::dsl::*;

    let conn = match state.database.get_connection().await {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let result = conn
        .interact(|conn| {
            config
                .filter(name.eq("client_id"))
                .get_result::<Config>(conn)
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|e| {
            tracing::error!("Error getting client ID: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(result.value)
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
