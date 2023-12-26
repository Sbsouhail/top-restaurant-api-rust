use std::sync::Arc;

use crate::{
    modules::{
        restaurants::restaurants_dto::{CreateRestaurant, Restaurant},
        shared::shared_dto::{AppResult, PaginatedList, PaginationInput},
        users::users_dto::User,
    },
    AppState,
};
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
};

use super::restaurants_dto::RestaurantFilters;

pub async fn get_restaurants(
    State(state): State<Arc<AppState>>,
    pagination_input: Query<PaginationInput>,
    filter_input: Query<RestaurantFilters>,
) -> AppResult<PaginatedList<Restaurant>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        Restaurant,
        "SELECT restaurant_id,name,is_accepted,user_id from restaurants where is_accepted = $1 LIMIT $2 OFFSET $3",
        filter_input.is_accepted,
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!(
        "SELECT COUNT (restaurant_id) FROM restaurants where is_accepted = $1",
        filter_input.is_accepted
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => count.unwrap_or(0),
        Err(_) => 0,
    };

    return match res {
        Ok(restaurants) => AppResult::Result(
            StatusCode::OK,
            PaginatedList {
                count,
                items: restaurants,
            },
        ),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}

pub async fn get_my_restaurants(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Arc<User>>,
    pagination_input: Query<PaginationInput>,
) -> AppResult<PaginatedList<Restaurant>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        Restaurant,
        "SELECT restaurant_id,name,is_accepted,user_id from restaurants where user_id = $1 LIMIT $2 OFFSET $3",
        current_user.user_id,
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!(
        "SELECT COUNT (restaurant_id) FROM restaurants where user_id = $1",
        current_user.user_id
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(count) => count.unwrap_or(0),
        Err(_) => 0,
    };

    return match res {
        Ok(restaurants) => AppResult::Result(
            StatusCode::OK,
            PaginatedList {
                count,
                items: restaurants,
            },
        ),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}

pub async fn create_restaurant(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Arc<User>>,
    Json(create_restaurant_dto): Json<CreateRestaurant>,
) -> AppResult<Restaurant> {
    let res = sqlx::query_as!(
        Restaurant,
        "INSERT INTO restaurants (name,user_id) VALUES ($1,$2) RETURNING restaurant_id,name,is_accepted,user_id",
        create_restaurant_dto.name,
        current_user.user_id
    )
    .fetch_one(&state.db)
    .await;

    return match res {
        Ok(restaurant) => AppResult::Result(StatusCode::CREATED, restaurant),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}

pub async fn accept_restaurant(
    State(state): State<Arc<AppState>>,
    Path(restaurant_id): Path<i32>,
) -> AppResult<Restaurant> {
    let res = sqlx::query_as!(
        Restaurant,
        "SELECT restaurant_id,name,is_accepted,user_id from restaurants where restaurant_id = $1 AND is_accepted = true",
        restaurant_id
    )
    .fetch_one(&state.db)
    .await;

    if let Ok(_) = res {
        return AppResult::Error(StatusCode::CONFLICT, String::from("Already accepted!"));
    }

    let res = sqlx::query_as!(
        Restaurant,
        "UPDATE restaurants SET is_accepted = true WHERE user_id = $1 RETURNING restaurant_id,name,is_accepted,user_id ",
        restaurant_id
    )
    .fetch_one(&state.db)
    .await;

    return match res {
        Ok(restaurant) => AppResult::Result(StatusCode::CREATED, restaurant),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}
