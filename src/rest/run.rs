use anyhow::Result;

use crate::{rest::routes::routes::routes, utils::addr::server_ip_address};

pub async fn run_rest() -> Result<()> {
    println!("Running REST API...");
    _ = tracing_subscriber::fmt::try_init();

    let app = routes().await;

    let addr = server_ip_address();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
