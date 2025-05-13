pub mod error;

use error::ConfigError;
use std::env;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type ConfigType = Arc<Mutex<Config>>;

#[derive(Debug, Clone)]
pub struct Config {
    pub ip_addr: IpAddr,
    pub port: u16,
    pub database_url: String,
    pub assets_path: String,
    pub index_path: String,
    pub irc_channel: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Config {
            ip_addr: env::var("IP")
                .unwrap_or_else(|_| "0.0.0.0".to_string())
                .parse()?,
            port: env::var("PORT")
                .unwrap_or_else(|_| "25000".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL").map_err(|e| ConfigError::EnvVarError {
                field: "DATABASE_URL".to_string(),
                source: e,
            })?,
            assets_path: env::var("ASSETS_PATH").unwrap_or_else(|_| "assets".to_string()),
            index_path: env::var("ASSETS_PATH").unwrap_or_else(|_| "assets".to_string()),
            irc_channel: env::var("IRC_CHANNEL").map_err(|e| ConfigError::EnvVarError {
                field: "IRC_CHANNEL".to_string(),
                source: e,
            })?,
        })
    }
}
