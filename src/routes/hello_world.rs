use axum::Extension;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, doc, Document},
    options::FindOptions,
    Database,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    _id: bson::oid::ObjectId,
    first_name: String,
    last_name: String,
    created_at: bson::DateTime,
    updated_at: bson::DateTime,
    email: String,
}

pub async fn hello_world(Extension(db): Extension<Database>) {
    let collection = db.collection::<Document>("User");

    let mut tracks: Vec<User> = vec![];
    let filter = doc! {};

    let find_options = FindOptions::builder().sort(doc! {}).build();
    let mut cursor = collection.find(filter, find_options).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(user) = cursor.try_next().await.unwrap() {
        let user: User = bson::from_document(user).unwrap();
        tracks.push(user);
    }
}
