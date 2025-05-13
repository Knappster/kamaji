use thiserror::Error;

use crate::config::error::ConfigError;
use crate::irc::error::IrcError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("config error: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("IO Error!")]
    IoError(#[from] std::io::Error),
    #[error("IRC Error!")]
    IrcError(#[from] IrcError),
}
