use axum::{Router, routing::{get, post}, Json, http::HeaderMap, extract::State};
use serde::Deserialize;
use sqlx::{postgres::{PgPool, PgPoolOptions}, Executor};
use std::env;

// Still not sure but nice idea
#[derive(Deserialize)]
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
    let distance_km = payload.get("distance");
    let time_minutes = payload.get("time");
    let user_id = payload.get("user_id");
    let rithms: Vec<String> = payload
        .get("rithms")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();
    // NEW
    let times: Vec<f64> = payload
        .get("times")
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .unwrap_or_default();
    let elevation_gain = payload.get("elevation_gain");
    let elevation_loss = payload.get("elevation_loss");

    // pool.execute(
    //     "INSERT INTO trainings (distance_km, time_minutes, user_id, rithms, times, elevation_gain, elevation_loss) VALUES ($1, $2, $3, $4, $5, $6, $7)",
    //     &[&distance_km, &time_minutes, &user_id, &rithms, &times, &elevation_gain, &elevation_loss],
    // );

    let result = sqlx::query!(
        r#"INSERT INTO races (distance, time, runner_id, rithms, times, elevation_gain, elevation_loss) 
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        returning id"#,
        &distance_km, &time_minutes, &user_id, &rithms, &times, &elevation_gain, &elevation_loss,

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
async fn get_activities(State(pool): State<PgPool>, header: HeaderMap) -> &'static str {
    // SHOULD ADD SOME SECURITY 
    let runner_id = header.get("runner_id");
    // No verification for now
    let offset = header.get("offset");

    // let activities = pool.fetch_all(
    //     "SELECT (distance_km, time_minutes, user_id, rithms, times, elevation_gain, elevation_loss) FROM trainings WHERE user_id = $1",
    //     &[&user_id] 
    // ).await;

    let activities = sqlx::query_as!(
        TrainingData,
        "SELECT * FROM races WHERE runner_id = $1",
        runner_id
    )
    .fetch_all(&pool)
    .await;

    match activities {
        Ok(activities) =>  {
            // Return activities as JSON
            let activities_json = serde_json::to_string(&activities).unwrap();
            activities_json.as_str()
        },
        Err(_) =>  "Failed to fetch activities"
    }

}