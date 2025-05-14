use std::env::VarError;
use std::net::AddrParseError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("ip_address_parsing: {0}")]
    IpAddressError(#[from] AddrParseError),
    #[error("port_parsing: {0}")]
    PortError(#[from] ParseIntError),
    #[error("environment_variable: {field} - {source}")]
    EnvVarError {
        field: String,
        #[source]
        source: VarError,
    },
}
