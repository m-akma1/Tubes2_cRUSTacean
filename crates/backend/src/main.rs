use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
struct ScrapeQuery {
    url: String,
}

#[derive(Debug, Serialize)]
struct ScrapeResponse {
    requested_url: String,
    final_url: String,
    status_code: u16,
    content_type: Option<String>,
    html: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

enum ApiError {
    BadRequest(String),
    Upstream(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Upstream(message) => (StatusCode::BAD_GATEWAY, message),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

fn validate_url(input: &str) -> Result<(), ApiError> {
    if input.trim().is_empty() {
        return Err(ApiError::BadRequest("URL is required.".to_string()));
    }

    if !(input.starts_with("http://") || input.starts_with("https://")) {
        return Err(ApiError::BadRequest(
            "URL must start with http:// or https://".to_string(),
        ));
    }

    Ok(())
}

async fn scrape(
    State(client): State<Client>,
    Query(query): Query<ScrapeQuery>,
) -> Result<Json<ScrapeResponse>, ApiError> {
    validate_url(&query.url)?;

    let response = client
        .get(&query.url)
        .send()
        .await
        .map_err(|error| ApiError::Upstream(format!("failed to fetch URL: {error}")))?;

    let status_code = response.status().as_u16();
    let final_url = response.url().to_string();
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);

    let html = response
        .text()
        .await
        .map_err(|error| ApiError::Upstream(format!("failed to read response body: {error}")))?;

    Ok(Json(ScrapeResponse {
        requested_url: query.url,
        final_url,
        status_code,
        content_type,
        html,
    }))
}

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .user_agent("crustacean-backend/0.1")
        .build()
        .expect("failed to build reqwest client");

    let app = Router::new()
        .route("/", get(|| async { "Backend entry point. Tes 1 2 3 aman yak." }))
        .route("/scrape", get(scrape))
        .layer(CorsLayer::permissive())
        .with_state(client);

    println!("Backend running on http://127.0.0.1:2026");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:2026").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
