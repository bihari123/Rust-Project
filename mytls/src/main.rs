//! Run with `cargo run --all-features --example http_and_https` command.
//!
//! To connect through browser, navigate to "http://localhost:3000" url which should redirect to
//! "https://localhost:3443".

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use serde::{Deserialize, Serialize};
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
        .route("/how", get(|| async { "\nHow you doing!\n" }))
        .route("/users", post(create_user))
        .route("/users", put(create_user));

    let config =
        RustlsConfig::from_pem_file("self-signed-certs/cert.pem", "self-signed-certs/key.pem")
            .await
            .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3443));
    println!("https listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
