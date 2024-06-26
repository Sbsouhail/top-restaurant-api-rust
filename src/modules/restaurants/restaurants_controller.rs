use std::sync::Arc;

use crate::{
    modules::{
        restaurants::restaurants_dto::{CreateRestaurant, Restaurant},
        shared::shared_dto::{AppResult, PaginatedList, PaginationInput},
        users::users_dto::User, files::files_controller::delete_file,
    },
    AppState,
};
use axum::{
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    };

use super::restaurants_dto::RestaurantUser;

pub async fn get_restaurants(
    State(state): State<Arc<AppState>>,
    pagination_input: Query<PaginationInput>,
) -> AppResult<PaginatedList<RestaurantUser>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        RestaurantUser,
        "SELECT 
        restaurants.restaurant_id, 
        restaurants.name, 
        restaurants.user_id,
        restaurants.location,       
        restaurants.cover_image_uri,       
        restaurants.phone,       
        restaurants.email,       
        users.email AS user_email
        FROM restaurants
        JOIN users ON restaurants.user_id = users.user_id        
        LIMIT $1 OFFSET $2",
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!("SELECT COUNT (restaurant_id) FROM restaurants",)
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
        "SELECT restaurant_id,name,user_id,location,cover_image_uri,phone,email from restaurants where user_id = $1 LIMIT $2 OFFSET $3",
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
        "INSERT INTO restaurants (name,user_id,location,cover_image_uri,phone,email) VALUES ($1,$2,$3,$4,$5,$6) RETURNING restaurant_id,name,user_id,location,cover_image_uri,phone,email",
        create_restaurant_dto.name,
        current_user.user_id,
        create_restaurant_dto.location,
        create_restaurant_dto.cover_image_uri,
        create_restaurant_dto.phone,
        create_restaurant_dto.email
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

pub async fn get_restaurant(
    State(state): State<Arc<AppState>>,
    Path(restaurant_id): Path<i32>,
) -> AppResult<Restaurant> {
    let res = sqlx::query_as!(
        Restaurant,
        "SELECT restaurant_id,name,user_id,location,cover_image_uri,phone,email from restaurants where restaurant_id = $1",
        restaurant_id,
    )
    .fetch_one(&state.db)
    .await;

    return match res {
        Ok(restaurant) => AppResult::Result(StatusCode::OK, restaurant),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}

pub async fn delete_restaurant(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Arc<User>>,
    Path(restaurant_id): Path<i32>,
) -> AppResult<Restaurant> {
    let res = match current_user.role.as_str() {
        "Admin" => 
            sqlx::query_as!(
                Restaurant,
                "DELETE FROM restaurants Where restaurant_id = $1 RETURNING restaurant_id,name,user_id,location,cover_image_uri,phone,email",
                restaurant_id
            )
            .fetch_one(&state.db)
            .await,
        

        _ => sqlx::query_as!(
            Restaurant,
            "DELETE FROM restaurants Where restaurant_id = $1 and user_id = $2 RETURNING restaurant_id,name,user_id,location,cover_image_uri,phone,email",
            restaurant_id,
            current_user.user_id
        )
        .fetch_one(&state.db)
        .await,
    };

    return match res {
        Ok(restaurant) => {
            delete_file(restaurant.cover_image_uri.clone());

            AppResult::Result(StatusCode::OK, restaurant)},
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}
