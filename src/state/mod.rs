pub mod error;

use axum::extract::FromRef;
use error::StateError;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::{database::Database, events::Events};

#[derive(Debug, Clone, FromRef)]
pub struct State {
    pub config: Arc<Config>,
    pub database: Arc<Database>,
    pub events: Arc<Mutex<Events>>,
}

impl State {
    pub async fn new() -> Result<Self, StateError> {
        let config = Arc::new(Config::new()?);
        let database = Arc::new(Database::new(config.clone()).await?);
        let events = Arc::new(Mutex::new(Events::new()?));

        Ok(State {
            config,
            database,
            events,
        })
    }
}
