use std::sync::Arc;

use axum::extract::Path;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::events::{Event, Events};
use crate::state::State as AppState;

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
        state.events.lock().await.publish(event).await;
    }

    Ok(Json(serde_json::json!({
        "id": id,
    })))
}

pub async fn test_one(State(events): State<Arc<Mutex<Events>>>) -> Result<(), StatusCode> {
    let _ = events
        .lock()
        .await
        .publish(Event {
            event_type: "test.one".to_string(),
            payload: serde_json::json!({"message": "test one success".to_string()}),
        })
        .await;

    Ok(())
}

pub async fn test_two(State(events): State<Arc<Mutex<Events>>>) -> Result<(), StatusCode> {
    let _ = events
        .lock()
        .await
        .publish(Event {
            event_type: "test.two".to_string(),
            payload: serde_json::json!({"message": "test two success".to_string()}),
        })
        .await;

    Ok(())
}
