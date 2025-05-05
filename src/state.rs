use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::ConfigType;
use crate::{database::Database, events::Events};

pub type StateType = Arc<Mutex<State>>;

#[derive(Clone)]
pub struct State {
    pub database: Database,
    pub events: Events,
}

impl State {
    pub async fn new(config: ConfigType) -> Self {
        State {
            database: Database::new(config).await,
            events: Events::new(),
        }
    }
}
