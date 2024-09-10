mod error;

use dotenv::dotenv;
use std::env;
use tokio::{net::TcpListener, signal};
use tracing::info;

use error::Result;
use heavy_metal_notifier::web::routes;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let mut addr = String::from("localhost:");
    addr.push_str(&env::var("SERVICE_PORT").unwrap());

    let listener = TcpListener::bind(&addr).await?;
    info!("Serving at http://{addr}");

    axum::serve(listener, routes().await.unwrap())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler.")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}
