use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("io::{0}")]
    Io(#[from] std::io::Error),
}
