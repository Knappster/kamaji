mod database;
//mod events;
mod events;
mod models;
mod routes;
mod schema;
mod services;
mod state;

use axum::Router;
use dotenvy::dotenv;
use state::AppState;
use std::env;
use std::net::IpAddr;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
    let state = AppState {
        ..Default::default()
    };

    // Log all events to console.
    {
        let mut events_receiver = state.events.lock().await.subscribe_all();

        tokio::spawn(async move {
            while let Ok(event) = events_receiver.recv().await {
                tracing::info!(
                    "Event triggered: {} with payload: {:?}",
                    event.event_type,
                    event.payload
                );
            }
        });
    }

    // Configure routing and start listening for connections.
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "25000".to_string())
        .parse()
        .expect("Invalid port number.");
    let ip_addr: IpAddr = env::var("IP")
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .parse()
        .expect("IP address invalid.");
    let router: Router = routes::routes().with_state(state);
    let addr: SocketAddr = SocketAddr::from((ip_addr, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, router.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
