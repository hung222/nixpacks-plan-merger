use nixpacks::nixpacks::plan::BuildPlan;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/merge", post(merge));

    let port = env::var("PORT")
        .unwrap_or("3002".to_string())
        .parse()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize, Clone, Debug)]
struct MergeReq {
    plans: Vec<BuildPlan>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MergeRes {
    plan: BuildPlan,
}

async fn merge(Json(payload): Json<MergeReq>) -> impl IntoResponse {
    tracing::info!("merging {} plans", payload.plans.len());

    let mut merged = BuildPlan::merge_plans(&payload.plans);
    merged.pin();

    (StatusCode::OK, Json(merged))
}
