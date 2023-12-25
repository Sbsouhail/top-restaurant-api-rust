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

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    pagination_input: Query<PaginationInput>,
) -> AppResult<PaginatedList<User>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        User,
        "SELECT user_id,name,last_name,email,role,is_stadium_owner_request from users LIMIT $1 OFFSET $2",
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!("SELECT COUNT (user_id) FROM users")
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
        "SELECT user_id,name,last_name,email,role,is_stadium_owner_request from users where user_id = $1",
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

pub async fn request_stadium_owner(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Arc<User>>,
) -> AppResult<User> {
    if current_user.is_stadium_owner_request == true {
        return AppResult::Error(StatusCode::CONFLICT, String::from("Already requested!"));
    }

    let res = sqlx::query_as!(
        User,
        "UPDATE users SET is_stadium_owner_request = true WHERE user_id = $1 RETURNING user_id,name,last_name,email,role,is_stadium_owner_request ",
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

pub async fn accept_stadium_owner(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> AppResult<User> {
    let res = sqlx::query_as!(
        User,
        "SELECT user_id,name,last_name,email,role,is_stadium_owner_request from users where user_id = $1 AND role = 'StadiumOwner'",
        user_id
    )
    .fetch_one(&state.db)
    .await;

    if let Ok(_) = res {
        return AppResult::Error(StatusCode::CONFLICT, String::from("Already accepted!"));
    }

    let res = sqlx::query_as!(
        User,
        "UPDATE users SET role = 'StadiumOwner' WHERE user_id = $1 RETURNING user_id,name,last_name,email,role,is_stadium_owner_request ",
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
