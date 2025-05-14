use thiserror::Error;
use tmi::client::read::RecvError;
use tmi::client::write::SendError;
use tmi::client::{ConnectError, ReconnectError};
use tmi::MessageParseError;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum IrcError {
    #[error("connection: {0}")]
    Connection(#[from] ConnectError),
    #[error("send: {0}")]
    Send(#[from] SendError),
    #[error("join: {0}")]
    Join(#[from] JoinError),
    #[error("receive: {0}")]
    Receiver(#[from] RecvError),
    #[error("message_parsing: {0}")]
    MessageParsing(#[from] MessageParseError),
    #[error("reconnect: {0}")]
    Reconnect(#[from] ReconnectError),
}
