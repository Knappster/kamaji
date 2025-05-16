use thiserror::Error;

use crate::{
    config::error::ConfigError, database::error::DatabaseError, events::error::EventsError,
};

#[derive(Error, Debug)]
pub enum StateError {
    #[error("config::{0}")]
    Config(#[from] ConfigError),
    #[error("database::{0}")]
    Database(#[from] DatabaseError),
    #[error("events::{0}")]
    Events(#[from] EventsError),
}
