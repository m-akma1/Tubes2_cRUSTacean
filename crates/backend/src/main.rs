use axum::{
    routing::get,
    Router
};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app = Router::new()
    .route("/", get(|| async {"Backend entry point. Tes 1 2 3 aman yak."}))
    .with_state(client);

    println!("Backend running on http://127.0.0.1:2026");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:2026").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
