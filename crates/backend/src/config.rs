use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub max_request_body_bytes: usize,
    pub scrape_max_redirects: usize,
    pub scrape_max_body_bytes: usize,
    pub app_env: String,
    pub cors_allowed_origins: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(2211),
            max_request_body_bytes: env_usize("MAX_REQUEST_BODY_BYTES", 2 * 1024 * 1024),
            scrape_max_redirects: env_usize("SCRAPE_MAX_REDIRECTS", 3),
            scrape_max_body_bytes: env_usize("SCRAPE_MAX_BODY_BYTES", 2 * 1024 * 1024),
            app_env: env::var("APP_ENV")
                .unwrap_or_else(|_| "development".to_string())
                .to_ascii_lowercase(),
            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS").unwrap_or_default(),
        }
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn env_usize(name: &str, fallback: usize) -> usize {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(fallback)
}