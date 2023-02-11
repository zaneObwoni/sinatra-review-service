use std::net::SocketAddr;

use axum::{routing::get, Router};

fn new_routes() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    let app = Router::new().nest("/api", new_routes());

    println!("Listening on http://[::1]:8000");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to run!");
}
