use thiserror::Error;
use tmi::client::read::RecvError;
use tmi::client::write::SendError;
use tmi::client::{ConnectError, ReconnectError};
use tmi::MessageParseError;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum IrcError {
    #[error("Connection Error!")]
    ConnectionError(#[from] ConnectError),
    #[error("Send Error!")]
    SendError(#[from] SendError),
    #[error("Join Error!")]
    JoinError(#[from] JoinError),
    #[error("Receive Error!")]
    ReceiverError(#[from] RecvError),
    #[error("Message Parsing Error!")]
    MessageParsingError(#[from] MessageParseError),
    #[error("Reconnect Error!")]
    ReconnectError(#[from] ReconnectError),
}
