use axum::{Router, routing::{get, post}, Json, http::HeaderMap, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::{PgPool, PgPoolOptions}, Executor};
use std::env;

// Still not sure but nice idea
#[derive(Deserialize, Serialize)]
struct TrainingData {
    // Implementation later w/ special library for geo points
    // vec_points: Vec<Point>,
    distance: f64,
    elevation_gain: f64,
    elevation_loss: f64,
    rithm: f64,
    time: f64,
    rithms: Vec<f64>,
    times: Vec<f64>,
    runner_id: i32,
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

#[axum::debug_handler]
async fn post_activity(State(pool): State<PgPool>, header: HeaderMap, Json(payload): Json<serde_json::Value>) -> &'static str {
    let distance_km = payload.get("distance").and_then(|value| value.as_f64()).unwrap_or(0.0);
    let time_minutes = payload.get("time").and_then(|value| value.as_f64()).unwrap_or(0.0);
    let user_id = payload.get("user_id").and_then(|value| value.as_i64()).unwrap_or(0) as i32;
    let rithm = payload.get("rithm").and_then(|value| value.as_f64()).unwrap_or(0.0);
    let rithms: Vec<f64> = payload
        .get("rithms")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();
    // NEW
    let times: Vec<f64> = payload
        .get("times")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();
    let elevation_gain = payload.get("elevation_gain").and_then(|value| value.as_f64()).unwrap_or(0.0);
    let elevation_loss = payload.get("elevation_loss").and_then(|value| value.as_f64()).unwrap_or(0.0);

    // pool.execute(
    //     "INSERT INTO trainings (distance_km, time_minutes, user_id, rithms, times, elevation_gain, elevation_loss) VALUES ($1, $2, $3, $4, $5, $6, $7)",
    //     &[&distance_km, &time_minutes, &user_id, &rithms, &times, &elevation_gain, &elevation_loss],
    // );

    let result = sqlx::query!(
        r#"INSERT INTO races (distance, time, runner_id, rithm, rithms, times, elevation_gain, elevation_loss) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        returning id"#,
        &distance_km, &time_minutes, &user_id, &rithm, &rithms, &times, &elevation_gain, &elevation_loss,

    )
    .fetch_one(&pool) 
    .await;

    match result {
        Ok(row) => {
            let id: i32 = row.id;
            println!("Inserted training with ID: {}", id);
        }
        Err(e) => {
            eprintln!("Failed to insert training: {}", e);
        }
    }
    
    "Post Activity"


}

#[axum::debug_handler]
async fn get_activities(State(pool): State<PgPool>, header: HeaderMap) -> Result<Json<Vec<TrainingData>>, StatusCode> {
    // SHOULD ADD SOME SECURITY 
    let runner_id: i32 = header.get("runner_id").and_then(|value| value.to_str().ok()).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
    // No verification for now
    let offset = header.get("offset");

    // let activities = pool.fetch_all(
    //     "SELECT (distance_km, time_minutes, user_id, rithms, times, elevation_gain, elevation_loss) FROM trainings WHERE user_id = $1",
    //     &[&user_id] 
    // ).await;

    let activities = sqlx::query_as!(
        TrainingData,
        "SELECT distance, time, runner_id, rithm, rithms, times, elevation_gain, elevation_loss FROM races WHERE runner_id = $1",
        runner_id
    )
    .fetch_all(&pool)
    .await;

    match activities {
        Ok(activities) => Ok(Json(activities)),
        Err(e) =>  Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}