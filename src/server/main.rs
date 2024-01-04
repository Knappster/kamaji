mod database;
mod routes;
mod state;

use dotenvy::dotenv;
use std::env;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::process::exit;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Configure environment variables.
    dotenv().ok();

    // Configure logging.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "kamaji=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create database connection.
    let db_pool = database::init_db_pool().await.unwrap_or_else(|e| {
        tracing::error!("{}", e);
        exit(1)
    });

    // Run any migrations.
    database::run_db_migrations(&db_pool)
        .await
        .unwrap_or_else(|e| {
            error!("{}", e);
            exit(1)
        });

    // Create app state.
    let state = AppState { database: db_pool };

    // Configure routing and start listening for connections.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "80".to_string())
        .parse::<u16>()
        .unwrap_or_else(|e| {
            tracing::error!("Invalid port number: {}", e);
            exit(1)
        });
    debug!(port);

    let ip_addr = env::var("IP")
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .parse::<IpAddr>()
        .unwrap_or_else(|e| {
            tracing::error!("Invalid IP address: {}", e);
            exit(1)
        });
    debug!("ip_address={}", ip_addr.to_string());

    let router = routes::get_routes(state);
    let addr = SocketAddr::from((ip_addr, port));

    info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        router.layer(TraceLayer::new_for_http()).into_make_service(),
    )
    .await
    .unwrap();
}
