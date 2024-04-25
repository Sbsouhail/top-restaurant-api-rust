use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateRestaurantMenuItem {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub cover_image_uri: String,
}

#[derive(Serialize, FromRow)]
pub struct RestaurantMenuItem {
    pub restaurant_menu_item_id: i64,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub restaurant_menu_id: i64,
    pub cover_image_uri: String,
}
