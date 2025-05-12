mod config;
mod database;
mod events;
mod http;
mod irc;
mod state;

use dotenvy::dotenv;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Mutex as TokioMutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::*;
use crate::http::*;
use crate::irc::*;
use crate::state::*;

#[tokio::main]
async fn main() {
    // Grab initial config from environment or .env file.
    dotenv().ok();

    // Configure logging.
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "kamaji=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Init config.
    let config = Arc::new(Mutex::new(Config::new()));

    // Configure app state.
    let state = Arc::new(TokioMutex::new(State::new(config.clone()).await));

    // Start services.
    let http = http_serve(config.clone(), state.clone());
    let twitch_irc = irc_connect(state.clone());

    tokio::select! {
        res = http => {
            if let Err(error) = res {
                tracing::error!("Axum failure: {:?}", error);
            }
        },
        res = twitch_irc => {
            if let Err(error) = res {
                tracing::error!("Twitch IRC failure: {:?}", error);
            }
        }
    }
}
