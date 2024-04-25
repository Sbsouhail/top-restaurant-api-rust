use std::sync::Arc;

use crate::{
    modules::{
        files::files_controller::delete_file,
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

use super::restaurant_menus_dto::{CreateRestaurantMenu, RestaurantMenu};

pub async fn get_restaurant_menus_pub(
    State(state): State<Arc<AppState>>,
    pagination_input: Query<PaginationInput>,
    Path(restaurant_id): Path<i32>,
) -> AppResult<PaginatedList<RestaurantMenu>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        RestaurantMenu,
        "SELECT 
        restaurant_menu_id, 
        name, 
        is_active,
        restaurant_id       
        FROM restaurant_menus
        where restaurant_id = $1 
        and is_active = true  
        ORDER BY
	    restaurant_menu_id asc
        LIMIT $2 OFFSET $3",
        restaurant_id,
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!(
        "SELECT COUNT (restaurant_id) FROM restaurant_menus where restaurant_id = $1 and is_active = true",
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
pub async fn get_restaurant_menus(
    State(state): State<Arc<AppState>>,
    pagination_input: Query<PaginationInput>,
    Path(restaurant_id): Path<i32>,
) -> AppResult<PaginatedList<RestaurantMenu>> {
    let PaginationInput { page, page_size } = pagination_input.0;
    let offset = (page.saturating_sub(1)) * page_size;
    let res = sqlx::query_as!(
        RestaurantMenu,
        "SELECT 
        restaurant_menu_id, 
        name, 
        is_active,
        restaurant_id       
        FROM restaurant_menus
        where restaurant_id = $1        
        ORDER BY
	    restaurant_menu_id asc
        LIMIT $2 OFFSET $3",
        restaurant_id,
        page_size,
        offset
    )
    .fetch_all(&state.db)
    .await;

    let count: i64 = match sqlx::query_scalar!(
        "SELECT COUNT (restaurant_id) FROM restaurant_menus where restaurant_id = $1",
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

pub async fn create_restaurant_menu(
    State(state): State<Arc<AppState>>,
    Path(restaurant_id): Path<i32>,
    Json(create_restaurant_menu_dto): Json<CreateRestaurantMenu>,
) -> AppResult<RestaurantMenu> {
    let res = sqlx::query_as!(
        RestaurantMenu,
        "INSERT INTO restaurant_menus (name,is_active,restaurant_id) VALUES ($1,$2,$3) RETURNING restaurant_menu_id,name,is_active,restaurant_id",
        create_restaurant_menu_dto.name,
        create_restaurant_menu_dto.is_active,
        restaurant_id,
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

pub async fn activate_restaurant_menu(
    State(state): State<Arc<AppState>>,
    Path((restaurant_id, restaurant_menu_id)): Path<(i32, i32)>,
) -> AppResult<RestaurantMenu> {
    let res = sqlx::query_as!(
        RestaurantMenu,
        "update restaurant_menus SET is_active = true where restaurant_menu_id = $1 RETURNING restaurant_menu_id,name,is_active,restaurant_id",
        restaurant_menu_id
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

pub async fn disactivate_restaurant_menu(
    State(state): State<Arc<AppState>>,
    Path((restaurant_id, restaurant_menu_id)): Path<(i32, i32)>,
) -> AppResult<RestaurantMenu> {
    let res = sqlx::query_as!(
        RestaurantMenu,
        "update restaurant_menus SET is_active = false where restaurant_menu_id = $1 RETURNING restaurant_menu_id,name,is_active,restaurant_id",
        restaurant_menu_id
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

pub async fn delete_restaurant_menu(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Arc<User>>,
    Path((restaurant_id, restaurant_menu_id)): Path<(i32, i32)>,
) -> AppResult<RestaurantMenu> {
    let res = match current_user.role.as_str() {
        "Admin" =>
            sqlx::query_as!(
                RestaurantMenu,
                "DELETE FROM restaurant_menus Where restaurant_menu_id = $1 RETURNING restaurant_menu_id,name,is_active,restaurant_id",
                restaurant_menu_id
            )
            .fetch_one(&state.db)
            .await,

        _ => sqlx::query_as!(
            RestaurantMenu,
            "DELETE FROM restaurant_menus Where restaurant_menu_id = $1 RETURNING restaurant_menu_id,name,is_active,restaurant_id",
            restaurant_menu_id,
        )
        .fetch_one(&state.db)
        .await,
    };

    return match res {
        Ok(restaurant_menu) => AppResult::Result(StatusCode::OK, restaurant_menu),
        Err(_) => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong"),
        ),
    };
}
