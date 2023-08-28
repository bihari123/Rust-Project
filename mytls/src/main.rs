//! Run with `cargo run --all-features --example http_and_https` command.
//!
//! To connect through browser, navigate to "http://localhost:3000" url which should redirect to
//! "https://localhost:3443".

use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use std::net::SocketAddr;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let https = tokio::spawn(https_server());

    // Ignore errors.
    let _ = tokio::join!(https);
}

async fn https_server() {
    let app = Router::new()
        .route("/", get(|| async { "\nHello, world!\n" }))
        .route("/how", get(|| async { "\nHow you doing!\n" }));

    let config = RustlsConfig::from_pem_file(
        "src/self-signed-certs/cert.pem",
        "src/self-signed-certs/key.pem",
    )
    .await
    .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3443));
    println!("https listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
