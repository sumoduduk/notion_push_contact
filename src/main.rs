mod router;

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_rust));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_rust() -> String {
    "Hello Rust Axum".to_owned()
}
