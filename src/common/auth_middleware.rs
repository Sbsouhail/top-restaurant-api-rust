use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::{
    modules::{shared::shared_dto::AppResult, users::users_dto::User},
    AppState,
};

use super::jwt::decode_jwt;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppResult<()>> {
    match get_token(&headers) {
        Some(token) => match decode_jwt(token.to_string(), &state.env.jwt_secret) {
            Some(claims) => {
                let res = sqlx::query_as!(
                    User,
                    "SELECT email,name,last_name,user_id,role,is_stadium_owner_request from users where user_id = $1",
                    claims.sub
                )
                .fetch_one(&state.db)
                .await;

                match res {
                    Ok(user) => {
                        request.extensions_mut().insert(Arc::new(user));
                        let response = next.run(request).await;
                        Ok(response)
                    }
                    Err(_) => Err(AppResult::Error(
                        StatusCode::UNAUTHORIZED,
                        String::from("Unauthorized! Invalid token."),
                    )),
                }
            }
            None => Err(AppResult::Error(
                StatusCode::UNAUTHORIZED,
                String::from("Unauthorized! Invalid token."),
            )),
        },
        None => Err(AppResult::Error(
            StatusCode::UNAUTHORIZED,
            String::from("Unauthorized! Missing token."),
        )),
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    if let Some(header) = headers.get("Authorization") {
        if let Ok(value) = header.to_str() {
            if value.starts_with("Bearer ") {
                return Some(value["Bearer ".len()..].trim());
            }
        }
    }
    None
}
