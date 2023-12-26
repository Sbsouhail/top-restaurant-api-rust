use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub is_restaurant_owner_request: bool,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub user_id: i32,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub is_restaurant_owner_request: bool,
}

pub enum RolesEnum {
    User,
    RestaurantOwner,
    Admin,
}

#[derive(Deserialize)]
pub struct UserFilters {
    #[serde(default = "default_status")]
    pub is_restaurant_owner_request: bool,
    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "User".to_string()
}
fn default_status() -> bool {
    true
}
