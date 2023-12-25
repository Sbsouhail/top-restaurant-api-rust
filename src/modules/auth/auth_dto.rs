use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LogIn {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LogedIn {
    pub token: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
