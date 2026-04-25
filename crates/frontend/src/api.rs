use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use shared::{
    LcaRequest, LcaResponse, ParseHtmlRequest, ParseHtmlResponse, ScrapeTreeResponse,
    TraverseRequest, TraverseResponse,
};

const API_BASE: &str = "/api";

fn parse_error_message(status: u16, status_text: &str, body: &str) -> String {
    if let Ok(value) = serde_json::from_str::<Value>(body) {
        if let Some(message) = value.get("error").and_then(|v| v.as_str()) {
            return format!("HTTP {}: {}", status, message);
        }
        if let Some(message) = value.get("message").and_then(|v| v.as_str()) {
            return format!("HTTP {}: {}", status, message);
        }
    }

    if !body.trim().is_empty() {
        format!("HTTP {}: {}", status, body)
    } else {
        format!("HTTP {}: {}", status, status_text)
    }
}

async fn read_json<T: DeserializeOwned>(response: Response) -> Result<T, String> {
    if response.ok() {
        response
            .json::<T>()
            .await
            .map_err(|e| format!("Failed to parse response JSON: {e}"))
    } else {
        let status = response.status();
        let status_text = response.status_text();
        let body = response.text().await.unwrap_or_default();
        Err(parse_error_message(status, &status_text, &body))
    }
}

pub async fn parse_html_tree(html: &str) -> Result<ParseHtmlResponse, String> {
    let payload = ParseHtmlRequest {
        html: html.to_string(),
        options: None,
    };

    let response = Request::post(&format!("{API_BASE}/parse-html"))
        .json(&payload)
        .map_err(|e| format!("Failed to build parse request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    read_json::<ParseHtmlResponse>(response).await
}

pub async fn scrape_tree(url: &str) -> Result<ScrapeTreeResponse, String> {
    let encoded_url = urlencoding::encode(url);
    let endpoint = format!("{API_BASE}/scrape?url={}&include_html=false", encoded_url);

    let response = Request::get(&endpoint)
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    read_json::<ScrapeTreeResponse>(response).await
}

pub async fn run_traverse(request: &TraverseRequest) -> Result<TraverseResponse, String> {
    let response = Request::post(&format!("{API_BASE}/traverse"))
        .json(request)
        .map_err(|e| format!("Failed to build traversal request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    read_json::<TraverseResponse>(response).await
}

pub async fn run_lca(request: &LcaRequest) -> Result<LcaResponse, String> {
    let response = Request::post(&format!("{API_BASE}/lca"))
        .json(request)
        .map_err(|e| format!("Failed to build LCA request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    read_json::<LcaResponse>(response).await
}
