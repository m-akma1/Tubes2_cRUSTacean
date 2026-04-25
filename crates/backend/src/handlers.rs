use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use shared::{
    AlgorithmKind, LcaRequest, LcaResponse, ParseHtmlRequest, ParseHtmlResponse,
    ScrapeTreeResponse, TraverseRequest, TraverseResponse,
};

use crate::{
    app::AppState,
    error::ApiError,
    parse::{build_tree_stats, map_parse_error, options_from_payload, options_from_query},
    scrape::{fetch_with_redirect_validation, read_response_body_limited},
    validator::parse_url,
};

#[derive(Debug, Deserialize)]
pub(crate) struct ScrapeQuery {
    url: String,
    include_html: Option<bool>,
    strict: Option<bool>,
    include_doctype: Option<bool>,
}

pub async fn scrape(
    State(state): State<AppState>,
    Query(query): Query<ScrapeQuery>,
) -> Result<Json<ScrapeTreeResponse>, ApiError> {
    let requested_url = parse_url(&query.url)?;

    let (response, final_url) = fetch_with_redirect_validation(
        &state.client,
        &requested_url,
        state.config.scrape_max_redirects,
    )
    .await?;

    let status_code = response.status().as_u16();
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string);

    let html = read_response_body_limited(response, state.config.scrape_max_body_bytes).await?;

    let options = options_from_query(query.strict, query.include_doctype);
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

pub async fn parse_html(
    Json(payload): Json<ParseHtmlRequest>,
) -> Result<Json<ParseHtmlResponse>, ApiError> {
    let options = options_from_payload(&payload);
    let tree = html_parser::parse_with_options(&payload.html, &options).map_err(map_parse_error)?;
    let stats = build_tree_stats(&tree);

    Ok(Json(ParseHtmlResponse { tree, stats }))
}

pub async fn traverse(Json(payload): Json<TraverseRequest>) -> impl IntoResponse {
    if let Err(error) = payload.tree.validate_integrity() {
        let response = TraverseResponse {
            result: None,
            message: Some(format!("Invalid tree: {error}")),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    let selector = match css_selector::parse(&payload.selector) {
        Ok(selector) => selector,
        Err(error) => {
            let response = TraverseResponse {
                result: None,
                message: Some(format!("Invalid selector: {error}")),
            };
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    let result = match (payload.algorithm, payload.parallel) {
        (AlgorithmKind::Bfs, true) => algorithm::bfs_parallel(&payload.tree, &selector, payload.top_n),
        (AlgorithmKind::Dfs, true) => algorithm::dfs_parallel(&payload.tree, &selector, payload.top_n),
        (AlgorithmKind::Bfs, false) => algorithm::bfs(&payload.tree, &selector, payload.top_n),
        (AlgorithmKind::Dfs, false) => algorithm::dfs(&payload.tree, &selector, payload.top_n),
    };

    let response = TraverseResponse {
        result: Some(result),
        message: None,
    };

    (StatusCode::OK, Json(response))
}

pub async fn lca(Json(payload): Json<LcaRequest>) -> impl IntoResponse {
    if let Err(error) = payload.tree.validate_integrity() {
        let response = LcaResponse {
            found: false,
            lca_index: None,
            message: Some(format!("Invalid tree: {error}")),
        };
        return (StatusCode::BAD_REQUEST, Json(response));
    }

    match algorithm::lca(&payload.tree, payload.node_a, payload.node_b) {
        Some(index) => {
            let response = LcaResponse {
                found: true,
                lca_index: Some(index),
                message: None,
            };
            (StatusCode::OK, Json(response))
        }
        None => {
            let response = LcaResponse {
                found: false,
                lca_index: None,
                message: Some("Invalid node indices or empty tree.".to_string()),
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}