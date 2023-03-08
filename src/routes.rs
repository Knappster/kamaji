use crate::models::Config;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_extra::routing::SpaRouter;
use axum_macros::debug_handler;
use diesel::prelude::*;

use crate::AppState;

#[debug_handler]
async fn get_client_id(State(state): State<AppState>) -> Result<String, AppError> {
    use crate::schema::config::dsl::*;

    let result = state
        .database
        .query_database(|conn| {
            config
                .filter(name.eq("client_id"))
                .get_result::<Config>(conn)
        })
        .await??;

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

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
