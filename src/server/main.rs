mod routes;
mod state;

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

    /*
    // Output all environment vars.
    #[cfg(debug_assertions)]
    {
        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }
    */

    // Configure logging.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "kamaji=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create app state.
    let state = AppState {
        ..Default::default()
    };

    // Configure routing and start listening for connections.
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "80".to_string())
        .parse()
        .expect("Invalid port number.");
    let ip_addr: IpAddr = env::var("IP")
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .parse()
        .expect("IP address invalid.");
    let router = routes::get_routes(state);
    let addr = SocketAddr::from((ip_addr, port));

    tracing::info!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        router.layer(TraceLayer::new_for_http()).into_make_service(),
    )
    .await
    .unwrap();
}
