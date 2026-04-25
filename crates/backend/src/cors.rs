use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

use crate::config::AppConfig;

pub fn build_cors_layer(config: &AppConfig) -> CorsLayer {
    let base = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT]);

    if config.app_env == "production" || config.app_env == "prod" {
        let origins = parse_allowed_origins(&config.cors_allowed_origins);
        if origins.is_empty() {
            eprintln!(
                "APP_ENV=production but CORS_ALLOWED_ORIGINS is empty; browser cross-origin access is denied by default."
            );
            return base;
        }
        return base.allow_origin(origins);
    }

    let mut origins = parse_allowed_origins(&config.cors_allowed_origins);
    if origins.is_empty() {
        origins = vec![
            HeaderValue::from_static("http://localhost:80"),
            HeaderValue::from_static("http://127.0.0.1:80"),
        ];
    }
    base.allow_origin(origins)
}

fn parse_allowed_origins(raw: &str) -> Vec<HeaderValue> {
    raw.split(',')
        .filter_map(|origin| {
            let trimmed = origin.trim();
            if trimmed.is_empty() {
                return None;
            }
            HeaderValue::from_str(trimmed).ok()
        })
        .collect()
}