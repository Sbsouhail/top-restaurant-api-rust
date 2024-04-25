use std::sync::Arc;

use crate::{
    modules::{
        files::files_controller::delete_file,
        restaurant_menu_items::restaurant_menu_items_dto::RestaurantMenuItem,
        restaurant_menus,
        shared::shared_dto::{AppResult, PaginatedList, PaginationInput},
        users::users_dto::User,
    },
    AppState,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    Extension,
};

use super::restaurant_menu_items_dto::CreateRestaurantMenuItem;

pub async fn get_restaurant_meals(
    State(state): State<Arc<AppState>>,
    pagination_input: Query<PaginationInput>,
    Path((restaurant_id)): Path<(i32)>,
) -> AppResult<PaginatedList<RestaurantMenuItem>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        RestaurantMenuItem,
        "SELECT 
        restaurant_menu_item_id, 
        rmi.name, 
        rmi.description,
        rmi.cover_image_uri,
        rmi.restaurant_menu_id,       
        rmi.price       
        FROM restaurant_menu_items rmi
        JOIN restaurant_menus rm ON rmi.restaurant_menu_id = rm.restaurant_menu_id
        WHERE rm.restaurant_id = $1       
        LIMIT $2 OFFSET $3",
        restaurant_id,
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!(
        "SELECT COUNT (restaurant_id) 
        FROM restaurant_menu_items rmi
        JOIN restaurant_menus rm ON rmi.restaurant_menu_id = rm.restaurant_menu_id
        WHERE rm.restaurant_id = $1 ",
        restaurant_id
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
pub async fn get_restaurant_menu_items(
    State(state): State<Arc<AppState>>,
    pagination_input: Query<PaginationInput>,
    Path((restaurant_id, restaurant_menu_id)): Path<(i32, i32)>,
) -> AppResult<PaginatedList<RestaurantMenuItem>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        RestaurantMenuItem,
        "SELECT 
        restaurant_menu_item_id, 
        name,
        price, 
        description,
        cover_image_uri,
        restaurant_menu_id       
        FROM restaurant_menu_items
        where restaurant_menu_id = $1        
        LIMIT $2 OFFSET $3",
        restaurant_menu_id,
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!(
        "SELECT COUNT (restaurant_menu_item_id) FROM restaurant_menu_items where restaurant_menu_id = $1 ",
        restaurant_menu_id
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

pub async fn create_restaurant_menu_item(
    State(state): State<Arc<AppState>>,
    Path((restaurant_id, restaurant_menu_id)): Path<(i32, i32)>,
    Json(create_restaurant_menu_item_dto): Json<CreateRestaurantMenuItem>,
) -> AppResult<RestaurantMenuItem> {
    let res = sqlx::query_as!(
        RestaurantMenuItem,
        "INSERT INTO restaurant_menu_items (name,description,restaurant_menu_id,cover_image_uri,price) VALUES ($1,$2,$3,$4,$5) RETURNING restaurant_menu_item_id,name,description,restaurant_menu_id,cover_image_uri,price",
        create_restaurant_menu_item_dto.name,
        create_restaurant_menu_item_dto.description,
        restaurant_menu_id,
        create_restaurant_menu_item_dto.cover_image_uri,
        create_restaurant_menu_item_dto.price,
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

pub async fn delete_restaurant_menu_item(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Arc<User>>,
    Path((restaurant_id, restaurant_menu_id, restaurant_menu_item_id)): Path<(i32, i32, i32)>,
) -> AppResult<RestaurantMenuItem> {
    let res = match current_user.role.as_str() {
        "Admin" =>
            sqlx::query_as!(
                RestaurantMenuItem,
                "DELETE FROM restaurant_menu_items Where restaurant_menu_item_id = $1 RETURNING restaurant_menu_item_id,name,description,restaurant_menu_id,cover_image_uri,price",
                restaurant_menu_item_id
            )
            .fetch_one(&state.db)
            .await,

        _ => sqlx::query_as!(
            RestaurantMenuItem,
            "DELETE FROM restaurant_menu_items Where restaurant_menu_item_id = $1 RETURNING restaurant_menu_item_id,name,description,restaurant_menu_id,cover_image_uri,price",
            restaurant_menu_item_id,
        )
        .fetch_one(&state.db)
        .await,
    };

    return match res {
        Ok(restaurant_menu_item) => {
            delete_file(restaurant_menu_item.cover_image_uri.clone());

            AppResult::Result(StatusCode::OK, restaurant_menu_item)
        }
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}
