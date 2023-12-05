use axum::{response::IntoResponse, routing::get, Router, http::Response, body::Full};
use axum::http::StatusCode;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/github-info", get(fetch_github_info));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn fetch_github_info() -> impl IntoResponse {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.github.com/users/ri-ch")
        .header("User-Agent", "GitHub V1")
        .send()
        .await
        .unwrap();

    let body: serde_json::Value = response.json().await.unwrap();

    Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/json")
    .body(Full::from(body.to_string()))
    .unwrap()
}
