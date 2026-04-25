use gloo_net::http::Request;
use serde_json::json;

/// POSTs `{ "url": url }` ke backend `/scrape` endpoint, return raw HTML.
/// Note: Backend pake alamat sama, so pake relative path
pub async fn scrape_url(url: &str) -> Result<String, String> {
    let body = json!({ "url": url });

    let response = Request::post("/scrape")
        .json(&body)
        .map_err(|e| format!("Failed to build request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if response.ok() {
        response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {e}"))
    } else {
        Err(format!(
            "Server returned HTTP {}: {}",
            response.status(),
            response.status_text()
        ))
    }
}
