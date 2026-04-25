use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router
};
use html_parser::{ParseError, ParseOptions};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use shared::{
    DomTree, LcaRequest, LcaResponse, ParseHtmlRequest, ParseHtmlResponse, ScrapeTreeResponse,
    TraverseRequest, TraverseResponse, TreeStats,
};
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
struct ScrapeQuery {
    url: String,
    include_html: Option<bool>,
    strict: Option<bool>,
    include_doctype: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Clone)]
struct AppState {
    client: Client,
}

enum ApiError {
    BadRequest(String),
    Unprocessable(String),
    Upstream(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Unprocessable(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
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

fn map_parse_error(error: ParseError) -> ApiError {
    match error {
        ParseError::EmptyInput => ApiError::BadRequest(error.to_string()),
        ParseError::UnmatchedEnd { .. }
        | ParseError::InvalidStructure(_)
        | ParseError::Tokenizer(_) => ApiError::Unprocessable(error.to_string()),
    }
}

fn build_tree_stats(tree: &DomTree) -> TreeStats {
    let edge_count = tree.nodes.iter().map(|node| node.children.len()).sum();

    TreeStats {
        node_count: tree.node_count(),
        edge_count,
        max_depth: tree.max_depth(),
    }
}

fn options_from_query(query: &ScrapeQuery) -> ParseOptions {
    ParseOptions {
        strict: query.strict.unwrap_or(false),
        include_doctype: query.include_doctype.unwrap_or(true),
    }
}

fn options_from_payload(payload: &ParseHtmlRequest) -> ParseOptions {
    let options = payload.options.clone().unwrap_or_default();
    ParseOptions {
        strict: options.strict,
        include_doctype: options.include_doctype,
    }
}

async fn scrape(
    State(state): State<AppState>,
    Query(query): Query<ScrapeQuery>,
) -> Result<Json<ScrapeTreeResponse>, ApiError> {
    validate_url(&query.url)?;

    let response = state
        .client
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

    let options = options_from_query(&query);
    let tree = html_parser::parse_with_options(&html, &options).map_err(map_parse_error)?;
    let stats = build_tree_stats(&tree);
    let html = if query.include_html.unwrap_or(false) {
        Some(html)
    } else {
        None
    };

    Ok(Json(ScrapeTreeResponse {
        requested_url: query.url,
        final_url,
        status_code,
        content_type,
        html,
        tree,
        stats,
    }))
}

async fn parse_html(
    Json(payload): Json<ParseHtmlRequest>,
) -> Result<Json<ParseHtmlResponse>, ApiError> {
    let options = options_from_payload(&payload);
    let tree = html_parser::parse_with_options(&payload.html, &options).map_err(map_parse_error)?;
    let stats = build_tree_stats(&tree);

    Ok(Json(ParseHtmlResponse { tree, stats }))
}

async fn traverse(Json(_payload): Json<TraverseRequest>) -> impl IntoResponse {
    let response = TraverseResponse {
        result: None,
        message: Some(
            "TUNGGU ALGORITMA Yak :)"
                .to_string(),
        ),
    };

    (StatusCode::NOT_IMPLEMENTED, Json(response))
}

async fn lca(Json(_payload): Json<LcaRequest>) -> impl IntoResponse {
    let response = LcaResponse {
        lca_index: None,
        message: Some(
            "TUNGGU ALGORITMA Yak :)"
                .to_string(),
        ),
    };

    (StatusCode::NOT_IMPLEMENTED, Json(response))
}

#[tokio::main]
async fn main() {
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .user_agent("crustacean-backend/0.1")
        .build()
        .expect("failed to build reqwest client");
    let state = AppState { client };

    let app = Router::new()
        .route("/", get(|| async { "Backend entry point. Tes 1 2 3 aman yak." }))
        .route("/scrape", get(scrape))
        .route("/parse-html", post(parse_html))
        .route("/traverse", post(traverse))
        .route("/lca", post(lca))
        .layer(CorsLayer::permissive())
        .with_state(state);

    println!("Backend running on http://127.0.0.1:2026");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:2026").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
