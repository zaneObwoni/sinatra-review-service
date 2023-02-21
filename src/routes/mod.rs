mod hello_world;

use axum::{http::Method, routing::get, Extension, Router};

use hello_world::hello_world;
use mongodb::Client;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes(db: Client) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(|| async { "Sinatra review service!" }))
        .route("/hello", get(hello_world))
        // make sure to add all routes above the database layer otherwise
        // anything below it will not have access to the database
        .layer(Extension(db))
        .layer(cors)
}
