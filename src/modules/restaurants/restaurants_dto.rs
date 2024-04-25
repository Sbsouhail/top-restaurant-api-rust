use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateRestaurant {
    pub name: String,
    pub location: String,
    pub cover_image_uri: String,
    pub phone: String,
    pub email: String,
}

#[derive(Serialize, FromRow)]
pub struct Restaurant {
    pub restaurant_id: i64,
    pub name: String,
    pub user_id: i64,
    pub location: String,
    pub cover_image_uri: String,
    pub phone: String,
    pub email: String,
}

#[derive(Serialize, FromRow)]
pub struct RestaurantUser {
    pub restaurant_id: i64,
    pub name: String,
    pub user_id: i64,
    pub location: String,
    pub phone: String,
    pub email: String,
    pub cover_image_uri: String,
    pub user_email: String,
}

#[derive(Deserialize)]
pub struct RestaurantFilters {
    #[serde(default = "default_status")]
    pub is_accepted: bool,
}

fn default_status() -> bool {
    true
}
