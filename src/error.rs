use thiserror::Error;

use crate::config::error::ConfigError;
use crate::http::error::HttpError;
use crate::irc::error::IrcError;
use crate::state::error::StateError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("config::{0}")]
    Config(#[from] ConfigError),
    #[error("state::{0}")]
    State(#[from] StateError),
    #[error("http::{0}")]
    Http(#[from] HttpError),
    #[error("irc::{0}")]
    Irc(#[from] IrcError),
}
