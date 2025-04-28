use axum::extract::FromRef;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{database::Database, events::Events};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub events: Arc<Mutex<Events>>,
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Database {
        app_state.database.clone()
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            database: Database::new(),
            events: Arc::new(Mutex::new(Events::new())),
        }
    }
}
