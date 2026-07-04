use std::net::SocketAddr;

use api_http::{app, init_tracing, listen_addr};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();

    let addr: SocketAddr = listen_addr().parse().unwrap_or_else(|err| {
        eprintln!("invalid listen address: {err}");
        std::process::exit(1);
    });

    let listener = TcpListener::bind(addr).await.unwrap_or_else(|err| {
        eprintln!("failed to bind {addr}: {err}");
        std::process::exit(1);
    });

    info!(%addr, "api-http listening");
    axum::serve(listener, app()).await.unwrap_or_else(|err| {
        eprintln!("server error: {err}");
        std::process::exit(1);
    });
}
