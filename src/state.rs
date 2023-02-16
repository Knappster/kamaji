use crate::database::Database;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            database: Database::new(),
        }
    }
}
