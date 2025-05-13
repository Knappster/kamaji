pub mod error;

use error::StateError;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::ConfigType;
use crate::{database::Database, events::Events};

pub type StateType = Arc<Mutex<State>>;

#[derive(Debug, Clone)]
pub struct State {
    pub database: Database,
    pub events: Events,
}

impl State {
    pub async fn new(config: ConfigType) -> Result<Self, StateError> {
        Ok(State {
            database: Database::new(config).await?,
            events: Events::new()?,
        })
    }
}
