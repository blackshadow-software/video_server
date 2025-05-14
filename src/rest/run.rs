use crate::{rest::routes::routes::routes, utils::addr::server_ip_address};
use anyhow::Result;

pub async fn run_rest() -> Result<()> {
    println!("Running REST API...");
    _ = tracing_subscriber::fmt::try_init();

    let app = routes().await;

    let addr = server_ip_address();

    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
