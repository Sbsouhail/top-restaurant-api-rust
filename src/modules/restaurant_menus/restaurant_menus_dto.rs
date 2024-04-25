use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateRestaurantMenu {
    pub name: String,
    pub is_active: bool,
}

#[derive(Serialize, FromRow)]
pub struct RestaurantMenu {
    pub restaurant_menu_id: i64,
    pub name: String,
    pub is_active: bool,
    pub restaurant_id: i64,
}
