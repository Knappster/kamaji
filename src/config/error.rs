use std::env::VarError;
use std::net::AddrParseError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("ip address parsing error: {0}")]
    IpAddressError(#[from] AddrParseError),
    #[error("port parsing error: {0}")]
    PortError(#[from] ParseIntError),
    #[error("environment variable error for {field}: {source}")]
    EnvVarError {
        field: String,
        #[source]
        source: VarError,
    },
}
