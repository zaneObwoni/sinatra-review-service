#[derive(Serialize, Deserialize)]
struct Review {
    id: i32,
    title: String,
    body: String,
    rating: i32,
    user_id: i32,
}

#[derive(Serialize, Deserialize)]
struct Reviewer {
    id: i32,
    name: String,
}
