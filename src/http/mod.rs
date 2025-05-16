pub mod error;
mod routes;
pub mod services;

use axum::Router;
use std::net::SocketAddr;
use tokio::signal;

use crate::State;
use error::HttpError;
use routes::*;

pub async fn http_serve(state: State) -> Result<(), HttpError> {
    let config = state.config.clone();

    let router: Router = routes(config.clone()).with_state(state);
    let addr: SocketAddr = SocketAddr::from((config.ip_addr, config.port));

    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("listening on {}", addr);

    axum::serve(listener, router)
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
}
