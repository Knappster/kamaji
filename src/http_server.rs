use axum::Router;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;

use crate::config::ConfigType;
use crate::routes::get_routes;
use crate::state::StateType;

pub async fn start_http_server(config: ConfigType, state: StateType) -> anyhow::Result<()> {
    let config = config.lock().unwrap();
    let state = state.lock().await.clone();

    let router: Router = get_routes().with_state(state);
    let addr: SocketAddr = SocketAddr::from((config.ip_addr, config.port));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port!");

    tracing::info!("Starting server.");
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, router.layer(TraceLayer::new_for_http()))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

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
