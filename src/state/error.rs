use thiserror::Error;

use crate::{database::error::DatabaseError, events::error::EventsError};

#[derive(Error, Debug)]
pub enum StateError {
    #[error("database::{0}")]
    Database(#[from] DatabaseError),
    #[error("events::{0}")]
    Events(#[from] EventsError),
}
