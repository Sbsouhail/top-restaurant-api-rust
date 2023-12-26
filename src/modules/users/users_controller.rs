use std::sync::Arc;

use crate::{
    modules::{
        shared::shared_dto::{AppResult, PaginatedList, PaginationInput},
        users::users_dto::User,
    },
    AppState,
};
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
};

use super::users_dto::UserFilters;

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    pagination_input: Query<PaginationInput>,
    filters_input: Query<UserFilters>,
) -> AppResult<PaginatedList<User>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        User,
        "SELECT user_id,name,last_name,email,role,is_restaurant_owner_request from users where is_restaurant_owner_request=$1 AND role=$2 LIMIT $3 OFFSET $4",
        filters_input.is_restaurant_owner_request,
        filters_input.role,
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!(
        "SELECT COUNT (user_id) FROM users where is_restaurant_owner_request=$1 AND role=$2",
        filters_input.is_restaurant_owner_request,
        filters_input.role,
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => count.unwrap_or(0),
        Err(_) => 0,
    };

    return match res {
        Ok(users) => AppResult::Result(
            StatusCode::OK,
            PaginatedList {
                count,
                items: users,
            },
        ),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}

pub async fn get_me(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Arc<User>>,
) -> AppResult<User> {
    let res = sqlx::query_as!(
        User,
        "SELECT user_id,name,last_name,email,role,is_restaurant_owner_request from users where user_id = $1",
        current_user.user_id
    )
    .fetch_one(&state.db)
    .await;

    return match res {
        Ok(user) => AppResult::Result(StatusCode::OK, user),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}

pub async fn request_restaurant_owner(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Arc<User>>,
) -> AppResult<User> {
    if current_user.is_restaurant_owner_request == true {
        return AppResult::Error(StatusCode::CONFLICT, String::from("Already requested!"));
    }

    let res = sqlx::query_as!(
        User,
        "UPDATE users SET is_restaurant_owner_request = true WHERE user_id = $1 RETURNING user_id,name,last_name,email,role,is_restaurant_owner_request ",
        current_user.user_id
    )
    .fetch_one(&state.db)
    .await;

    return match res {
        Ok(user) => AppResult::Result(StatusCode::CREATED, user),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}

pub async fn accept_restaurant_owner(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> AppResult<User> {
    let res = sqlx::query_as!(
        User,
        "SELECT user_id,name,last_name,email,role,is_restaurant_owner_request from users where user_id = $1 AND role = 'RestaurantOwner'",
        user_id
    )
    .fetch_one(&state.db)
    .await;

    if let Ok(_) = res {
        return AppResult::Error(StatusCode::CONFLICT, String::from("Already accepted!"));
    }

    let res = sqlx::query_as!(
        User,
        "UPDATE users SET role = 'RestaurantOwner' WHERE user_id = $1 RETURNING user_id,name,last_name,email,role,is_restaurant_owner_request ",
        user_id
    )
    .fetch_one(&state.db)
    .await;

    return match res {
        Ok(user) => AppResult::Result(StatusCode::CREATED, user),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}
