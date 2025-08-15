use std::{env, io};

use axum::Router;
use tokio::net::TcpListener;
use tracing::log::LevelFilter;

#[tokio::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let router = Router::new();

    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    tracing::info!("Listener bound on port {port}");
    axum::serve(listener, router).await
}
