mod config;
mod database;
mod events;
mod http_server;
mod logging;
mod routes;
mod services;
mod state;
mod twitch_irc;

use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Mutex as TokioMutex;

use crate::config::Config;
use crate::http_server::start_http_server;
use crate::logging::init_logging;
use crate::state::State;
use crate::twitch_irc::start_twitch_irc;

#[tokio::main]
async fn main() {
    /*
    #[cfg(debug_assertions)]
    {
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }
    */

    // Configure console logging.
    init_logging();

    // Init config.
    let config = Arc::new(Mutex::new(Config::new()));

    // Configure app state.
    let state = Arc::new(TokioMutex::new(State::new(config.clone()).await));

    // Start services.
    let http_server_future = start_http_server(config.clone(), state.clone());
    let twitch_irc_future = start_twitch_irc(state.clone());

    tokio::select! {
        res = http_server_future => {
            if let Err(error) = res {
                tracing::error!("Axum failure: {:?}", error);
            }
        },
        res = twitch_irc_future => {
            if let Err(error) = res {
                tracing::error!("Twitch IRC failure: {:?}", error);
            }
        }
    }
}
