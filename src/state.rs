use axum::extract::FromRef;

use crate::database::Database;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
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
        }
    }
}
