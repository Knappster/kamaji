use axum::extract::Path;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use serde_json::Value;

use crate::events::Event;
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
    let _ = state
        .events
        .lock()
        .await
        .publish(Event {
            event_type: "test.one".to_string(),
            payload: serde_json::json!({"message": "test one success".to_string()}),
        })
        .await;

    Ok(())
}

pub async fn test_two(State(state): State<AppState>) -> Result<(), StatusCode> {
    let _ = state
        .events
        .lock()
        .await
        .publish(Event {
            event_type: "test.two".to_string(),
            payload: serde_json::json!({"message": "test two success".to_string()}),
        })
        .await;

    Ok(())
}
