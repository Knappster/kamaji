use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{database::Database, events::Events};

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub events: Arc<Mutex<Events>>,
}

impl AppState {
    pub async fn new() -> Self {
        AppState {
            database: Database::new().await,
            events: Arc::new(Mutex::new(Events::new())),
        }
    }
}
