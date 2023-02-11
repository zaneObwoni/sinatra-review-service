mod routes;

use mongodb::Client;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    let client = Client::with_uri_str(
        "mongodb+srv://zane:zane@cluster0.fdmuo.mongodb.net/?retryWrites=true&w=majority",
    )
    .await
    .expect("Failed to initialize client!");

    let database = client.database("sinatra");

    let app = routes::create_routes(database).await;

    println!("Listenings on http://[::1]:8000");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to run!");
}
