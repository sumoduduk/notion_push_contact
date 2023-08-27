mod router;

use axum::{
    routing::{get, post},
    Router,
};
use router::{api_porto, push_contact};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(hello_rust))
        .route("/contact_form", post(push_contact))
        .route("/get_porto", get(api_porto))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_rust() -> String {
    "Hello World Rust Axum".to_owned()
}
