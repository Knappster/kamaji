mod config;
mod database;
mod error;
mod events;
mod http;
mod irc;
mod state;

use dotenvy::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::*;
use crate::error::*;
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

    if let Err(error) = run().await {
        tracing::error!("{}", error);
        tracing::debug!("{:?}", error);
    }
}

async fn run() -> Result<(), AppError> {
    // Init config.
    let config = Arc::new(TokioMutex::new(Config::new()?));

    // Configure app state.
    let state = Arc::new(TokioMutex::new(State::new(config.clone()).await?));

    // Start services.
    let http = http_serve(config.clone(), state.clone());
    let twitch_irc = irc_connect(config.clone(), state.clone());

    tokio::select! {
        result = http => result.map_err(AppError::from),
        result = twitch_irc => result.map_err(AppError::from)
    }
}
