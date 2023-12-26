use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateRestaurant {
    pub name: String,
}

#[derive(Serialize, FromRow)]
pub struct Restaurant {
    pub restaurant_id: i64,
    pub name: String,
    pub is_accepted: bool,
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct RestaurantFilters {
    #[serde(default = "default_status")]
    pub is_accepted: bool,
}

fn default_status() -> bool {
    true
}
