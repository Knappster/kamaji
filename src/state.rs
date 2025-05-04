use std::sync::Arc;
use std::sync::Mutex;

use crate::config::Config;
use crate::{database::Database, events::Events};

#[derive(Clone)]
pub struct State {
    pub database: Database,
    pub events: Events,
}

impl State {
    pub async fn new(config: Arc<Mutex<Config>>) -> Self {
        State {
            database: Database::new(config).await,
            events: Events::new(),
        }
    }
}
