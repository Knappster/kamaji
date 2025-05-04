mod database;
//mod events;
mod events;
mod routes;
mod services;
mod state;
mod twitch_irc;

use axum::serve::Listener;
use axum::Router;
use dotenvy::dotenv;
use state::AppState;
use std::env;
use std::future::IntoFuture;
use std::net::IpAddr;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler!");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler!")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Signal received, starting graceful shutdown!");
}

#[tokio::main]
async fn main() {
    // Configure environment variables.
    dotenv().ok();

    #[cfg(debug_assertions)]
    {
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }

    // Configure console logging.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "kamaji=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Configure app state.
    let state = AppState::new().await;

    // Configure routing and start listening for connections.
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "25000".to_string())
        .parse()
        .expect("Invalid port number.");
    let ip_addr: IpAddr = env::var("IP")
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .parse()
        .expect("IP address invalid.");
    let router: Router = routes::routes().with_state(state.clone());
    let addr: SocketAddr = SocketAddr::from((ip_addr, port));

    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            let axum_future = axum::serve(listener, router.layer(TraceLayer::new_for_http()))
                .with_graceful_shutdown(shutdown_signal())
                .into_future();
            tracing::info!("Listening on {}", addr);

            let twitch_irc_future = twitch_irc::twitch_chat(state.clone());

            tokio::select! {
                res = axum_future => {
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
        Err(error) => {
            tracing::error!("Failed to bind to port {:?}", error);
        }
    }
}
