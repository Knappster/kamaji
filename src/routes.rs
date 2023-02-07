use crate::models::Config;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use axum_extra::routing::SpaRouter;
use diesel::prelude::*;

use crate::AppState;

async fn get_client_id(State(state): State<AppState>) -> Result<String, StatusCode> {
    use crate::schema::config::dsl::*;

    let pool = &mut state.db_pool.get().await;
    let conn = match pool {
        Ok(conn) => conn,
        Err(_error) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let result = conn
        .interact(|conn| {
            config
                .filter(name.eq("client_id"))
                .get_result::<Config>(conn)
        })
        .await;

    match result {
        Ok(result) => match result {
            Ok(result) => Ok(result.value),
            Err(_error) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_error) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
