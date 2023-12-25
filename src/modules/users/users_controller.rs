use std::sync::Arc;

use crate::{
    modules::{
        shared::shared_dto::{AppResult, PaginatedList, PaginationInput},
        users::users_dto::User,
    },
    AppState,
};
use axum::{
    extract::{Extension, Query, State},
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
