use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
pub struct PaginatedList<T> {
    pub count: i64,
    pub items: Vec<T>,
}

#[derive(Deserialize)]
pub struct PaginationInput {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    10
}

pub enum AppResult<T> {
    Result(StatusCode, T),
    Error(StatusCode, String),
}

impl<T> IntoResponse for AppResult<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response<Body> {
        match self {
            AppResult::Result(status_code, data) => {
                (status_code, Json(json!(data))).into_response()
            }
            AppResult::Error(status_code, message) => {
                (status_code, Json(json!({ "message":message }))).into_response()
            }
        }
    }
}
