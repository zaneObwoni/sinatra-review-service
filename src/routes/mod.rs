mod hello_world;

use axum::{http::Method, routing::get, Router};
use hello_world::hello_world;
use mongodb::Database;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes(db: Database) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(|| async { "Sinatra review service!" }))
        .route("/hello", get(hello_world))
        .layer(cors)
}
