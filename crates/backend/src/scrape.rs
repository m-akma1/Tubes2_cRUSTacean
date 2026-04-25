use reqwest::{Client, Url};

use crate::{error::ApiError, validator::validate_target_url};

pub async fn fetch_with_redirect_validation(
    client: &Client,
    requested_url: &Url,
    max_redirects: usize,
) -> Result<(reqwest::Response, String), ApiError> {
    let mut current_url = requested_url.clone();

    for redirect_count in 0..=max_redirects {
        validate_target_url(&current_url).await?;

        let response = client
            .get(current_url.clone())
            .send()
            .await
            .map_err(|error| ApiError::Upstream(format!("failed to fetch URL: {error}")))?;

        if response.status().is_redirection() {
            if redirect_count == max_redirects {
                return Err(ApiError::Upstream(format!(
                    "too many redirects (maximum: {max_redirects})"
                )));
            }

            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .ok_or_else(|| {
                    ApiError::Upstream(
                        "upstream redirect response is missing Location header".to_string(),
                    )
                })?
                .to_str()
                .map_err(|error| {
                    ApiError::Upstream(format!("upstream redirect Location is invalid: {error}"))
                })?;

            current_url = current_url.join(location).map_err(|error| {
                ApiError::Upstream(format!("failed to resolve redirect URL: {error}"))
            })?;
            continue;
        }

        return Ok((response, current_url.to_string()));
    }

    Err(ApiError::Upstream(
        "failed to resolve final URL".to_string(),
    ))
}

pub async fn read_response_body_limited(
    mut response: reqwest::Response,
    max_body_bytes: usize,
) -> Result<String, ApiError> {
    let mut body = Vec::new();

    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| ApiError::Upstream(format!("failed to read response body: {error}")))?
    {
        if body.len().saturating_add(chunk.len()) > max_body_bytes {
            return Err(ApiError::Upstream(format!(
                "upstream response body exceeds limit of {max_body_bytes} bytes"
            )));
        }
        body.extend_from_slice(&chunk);
    }

    Ok(String::from_utf8_lossy(&body).into_owned())
}