use dotenvy::dotenv;
use std::env;
use std::net::IpAddr;

#[derive(Clone)]
pub struct Config {
    pub ip_addr: IpAddr,
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn new() -> Self {
        // Grab initial config from environment or .env file.
        dotenv().ok();

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
