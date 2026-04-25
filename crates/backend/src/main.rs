use reqwest::Client;
use std::time::Duration;

mod app;
mod config;
mod cors;
mod error;
mod handlers;
mod parse;
mod scrape;
mod validator;

use crate::{
    app::{AppState, build_app},
    config::AppConfig,
};

#[tokio::main]
async fn main() {
    let config = AppConfig::from_env();

    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::none())
        .user_agent("crustacean-backend/0.1")
        .build()
        .expect("failed to build reqwest client");
    let state = AppState { client, config };
    let app = build_app(state.clone());
    let bind_addr = state.config.bind_addr();

    println!("Backend running on http://{bind_addr}");
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind backend listener");
    axum::serve(listener, app).await.unwrap();
}
