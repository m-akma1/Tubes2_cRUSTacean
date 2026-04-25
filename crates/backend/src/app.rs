use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use reqwest::Client;

use crate::{
    config::AppConfig,
    cors::build_cors_layer,
    handlers::{lca, parse_html, scrape, traverse},
};

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub config: AppConfig,
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { "Backend entry point. Tes 1 2 3 aman yak." }),
        )
        .route("/scrape", get(scrape))
        .route("/parse-html", post(parse_html))
        .route("/traverse", post(traverse))
        .route("/lca", post(lca))
        .layer(DefaultBodyLimit::max(state.config.max_request_body_bytes))
        .layer(build_cors_layer(&state.config))
        .with_state(state)
}