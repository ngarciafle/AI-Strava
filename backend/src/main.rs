use axum::{Router, routing::{get, post}, Json, http::HeaderMap};
use serde::Deserialize;

// Still not sure but nice idea
#[derive(Deserialize)]
struct TrainingData {
    distance_km: f32,
    time_minutes: f32,
}

#[tokio::main]
async fn main() {
    // Accounts maybe in the future....
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/health", get(|| async { "OK" }))
    .route("/api/activities", post(post_activity).get(get_activities));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn post_activity(header: HeaderMap, Json(payload): Json<serde_json::Value>) -> &'static str {
    
    "Post Activity"
}

async fn get_activities(header: HeaderMap) -> &'static str {
    "Get Activities"
}