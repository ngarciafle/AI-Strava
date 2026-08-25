use axum::{Router, routing::{get, post}, Json, http::HeaderMap};
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

// Still not sure but nice idea
#[derive(Deserialize)]
struct TrainingData {
    distance_km: f32,
    time_minutes: f32,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to the database");
    
        // Accounts maybe in the future....
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/health", get(|| async { "OK" }))
    .route("/api/activities", post(post_activity).get(get_activities))
    .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn post_activity(State(pool): State<PgPool>, header: HeaderMap, Json(payload): Json<serde_json::Value>) -> &'static str {
    let distance_km = payload.get("distance_km");
    let time_minutes = payload.get("time_minutes");
    let user_id = payload.get("user_id");
    let rithms: Vec<String> = payload.get("rithms");
    // Route needs more work

    pool.execute(
        "INSERT INTO trainings (distance_km, time_minutes, user_id, rithms) VALUES ($1, $2, $3, $4)",
        &[&distance_km, &time_minutes, &user_id, &rithms],
    );
    
    "Post Activity"


}

async fn get_activities(State(pool): State<PgPool>, header: HeaderMap) -> &'static str {
    // No public search for now
    let user_id = header.get("user_id");
    // No verification for now

    let activities = pool.fetch_all(
        "SELECT * FROM trainings WHERE user_id = $1",
        &[&user_id] 
    ).await;

    return activities {
        Ok(activities) =>  {
            // Return activities as JSON
            let activities_json = serde_json::to_string(&activities).unwrap();
            activities_json.as_str()
        },
        Err(_) =>  "Failed to fetch activities"
    };

}