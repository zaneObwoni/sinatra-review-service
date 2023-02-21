mod routes;

use mongodb::Client;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8000));

    let uri = std::env::var("MONGODB_URI").expect("Failed to connect to MongoDB!");

    let client = Client::with_uri_str(uri).await.expect("Failed to connect");

    let app = routes::create_routes(client.clone()).await;

    println!("Listenings on http://[::1]:8080");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to run!");
}
