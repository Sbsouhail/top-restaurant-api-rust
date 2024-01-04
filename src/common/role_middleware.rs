use std::sync::Arc;

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Extension};

use crate::modules::{
    shared::shared_dto::AppResult,
    users::users_dto::{RolesEnum, User},
};

pub async fn role_middleware(
    Extension(current_user): Extension<Arc<User>>,
    request: Request,
    next: Next,
    accepted_role: RolesEnum,
) -> Result<Response, AppResult<()>> {
    match current_user.role.as_str() {
        "Admin" => {
            let response = next.run(request).await;
            Ok(response)
        }
        "RestaurantOwner" => match accepted_role {
            RolesEnum::RestaurantOwner => {
                let response = next.run(request).await;
                Ok(response)
            }
            _ => Err(AppResult::Error(
                StatusCode::FORBIDDEN,
                String::from("Forbidden resources!"),
            )),
        },
        "User" => match accepted_role {
            RolesEnum::User => {
                let response = next.run(request).await;
                Ok(response)
            }
            _ => Err(AppResult::Error(
                StatusCode::FORBIDDEN,
                String::from("Forbidden resources!"),
            )),
        },
        _ => Err(AppResult::Error(
            StatusCode::FORBIDDEN,
            String::from("Forbidden resources!"),
        )),
    }
}
