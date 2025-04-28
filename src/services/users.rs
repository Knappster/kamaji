use axum::extract::Path;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use diesel::prelude::*;
use serde_json::Value;

use crate::events::Event;
use crate::models::Config;
use crate::state::AppState;

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let event = Event {
        event_type: "user.get".to_string(),
        payload: serde_json::json!({
            "id": id
        }),
    };

    {
        let _ = state.events.lock().await.publish(event).await;
    }

    Ok(Json(serde_json::json!({
        "id": id,
    })))
}

pub async fn test_one(State(state): State<AppState>) -> Result<(), StatusCode> {
    let event = Event {
        event_type: "test.one".to_string(),
        payload: serde_json::json!({"message": "test one success".to_string()}),
    };

    let _ = state.events.lock().await.publish(event).await;

    Ok(())
}

pub async fn test_two(State(state): State<AppState>) -> Result<(), StatusCode> {
    let event = Event {
        event_type: "test.two".to_string(),
        payload: serde_json::json!({"message": "test two success".to_string()}),
    };

    let _ = state.events.lock().await.publish(event).await;

    Ok(())
}

pub async fn get_client_id(State(state): State<AppState>) -> Result<String, StatusCode> {
    use crate::schema::config::dsl::*;

    let conn = state.database.get_connection().await.map_err(|e| {
        tracing::error!("Error getting database connection: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

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
