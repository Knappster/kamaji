pub mod error;

use std::env;
use std::net::IpAddr;

use error::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    pub ip_addr: IpAddr,
    pub port: u16,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub auth_url: String,
    pub token_url: String,
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
            irc_channel: env::var("IRC_CHANNEL").map_err(|e| ConfigError::EnvVarError {
                field: "IRC_CHANNEL".to_string(),
                source: e,
            })?,
            client_id: env::var("CLIENT_ID").map_err(|e| ConfigError::EnvVarError {
                field: "CLIENT_ID".to_string(),
                source: e,
            })?,
            client_secret: env::var("CLIENT_SECRET").map_err(|e| ConfigError::EnvVarError {
                field: "CLIENT_SECRET".to_string(),
                source: e,
            })?,
            redirect_url: env::var("REDIRECT_URL")
                .unwrap_or_else(|_| "http://localhost:25000/auth/authorized".to_string()),

            auth_url: env::var("AUTH_URL").unwrap_or_else(|_| {
                "https://id.twitch.tv/oauth2/authorize?response_type=code".to_string()
            }),
            token_url: env::var("TOKEN_URL")
                .unwrap_or_else(|_| "https://id.twitch.tv/oauth2/token".to_string()),
            database_url: env::var("DATABASE_URL").map_err(|e| ConfigError::EnvVarError {
                field: "DATABASE_URL".to_string(),
                source: e,
            })?,
            assets_path: env::var("ASSETS_PATH").unwrap_or_else(|_| "assets".to_string()),
            index_path: env::var("ASSETS_PATH").unwrap_or_else(|_| "assets".to_string()),
        })
    }
}
