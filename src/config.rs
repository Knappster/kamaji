use std::env;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;

pub type ConfigType = Arc<Mutex<Config>>;

#[derive(Clone)]
pub struct Config {
    pub ip_addr: IpAddr,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ip_addr: env::var("IP")
                .unwrap_or_else(|_| "0.0.0.0".to_string())
                .parse()
                .expect("IP address invalid!"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "25000".to_string())
                .parse()
                .expect("Invalid port number!"),
            database_url: env::var("DATABASE_URL").expect("Missing database URL!"),
        }
    }
}
