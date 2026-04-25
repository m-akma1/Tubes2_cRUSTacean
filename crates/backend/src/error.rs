use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};


#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    error: String,
}

pub enum ApiError {
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