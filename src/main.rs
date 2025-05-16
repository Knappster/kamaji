mod config;
mod database;
mod error;
mod events;
mod http;
mod irc;
mod state;

use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    tracing::info!("kamaji started: v{}", VERSION);

    if let Err(error) = run().await {
        tracing::error!("{}", error);
        tracing::debug!("{:?}", error);
    }
}

async fn run() -> Result<(), AppError> {
    // Create default app state.
    let state = State::new().await?;

    // Start services.
    let http = http_serve(state.clone());
    let twitch_irc = irc_connect(state.clone());

    tokio::select! {
        result = http => result.map_err(AppError::from),
        result = twitch_irc => result.map_err(AppError::from)
    }
}
