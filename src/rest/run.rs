use crate::{rest::routes::routes::routes, utils::addr::server_ip_address};
use anyhow::Result;
use axum_server::tls_rustls::RustlsConfig;
use std::path::PathBuf;

pub async fn run_rest() -> Result<()> {
    println!("Running REST API...");
    _ = tracing_subscriber::fmt::try_init();

    let app = routes().await;

    let addr = server_ip_address();
    let cert = PathBuf::from("src/cert/cert.pem");
    let key = PathBuf::from("src/cert/key.pem");

    println!("Listening on {}", addr);

    if cert.exists() && key.exists() {
        println!("Using TLS");
        let tls = RustlsConfig::from_pem_file(cert, key).await?;
        axum_server::bind_rustls(addr, tls)
            .serve(app.into_make_service())
            .await?;
    } else {
        println!("TLS files not found, running without TLS");
        axum_server::bind(addr)
            .serve(app.into_make_service())
            .await?;
    }

    Ok(())
}
