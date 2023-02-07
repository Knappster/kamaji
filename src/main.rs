mod models;
mod routes;
mod schema;

use axum::Router;
use deadpool_diesel::mysql::{Manager, Pool, Runtime};
use dotenvy::dotenv;
use std::env;
use std::net::IpAddr;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    db_pool: Pool,
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

    // Configure database connection.
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(database_url, Runtime::Tokio1);
    let db_pool = Pool::builder(manager)
        .max_size(8)
        .build()
        .expect("Could not build database connection pool.");

    // Configure app state.
    let state = AppState { db_pool };

    // Configure routing and start listening for connections.
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "25000".to_string())
        .parse()
        .expect("Invalid port number.");
    let ip_addr: IpAddr = env::var("IP")
        .unwrap_or_else(|_| "0.0.0.0".to_string())
        .parse()
        .expect("IP address invalid.");
    let router: Router = routes::get_router(state);
    let addr: SocketAddr = SocketAddr::from((ip_addr, port));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
