use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use pwhash::{bcrypt, unix};

use crate::{
    common::jwt::{encode_jwt, Claims},
    modules::{
        shared::shared_dto::AppResult,
        users::users_dto::{User, UserModel},
    },
    AppState,
};

use super::auth_dto::{LogIn, LogedIn, RegisterUser};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(login_dto): Json<LogIn>,
) -> AppResult<LogedIn> {
    let res = sqlx::query_as!(
        UserModel,
        "SELECT email,password_hash,user_id,role,email_validated from users where email=$1 AND role = 'User'",
        login_dto.email
    )
    .fetch_one(&state.db)
    .await;

    if let Err(_) = res {
        return AppResult::Error(StatusCode::UNAUTHORIZED, String::from("Unauthorized!"));
    }

    let user = res.unwrap();

    if unix::verify(login_dto.password, &user.password_hash) == false {
        return AppResult::Error(StatusCode::UNAUTHORIZED, String::from("Unauthorized!"));
    }

    let now = chrono::Utc::now();
    let minutes: i64 = state.env.jwt_expires_in.parse().unwrap();
    let exp = (now + chrono::Duration::minutes(minutes)).timestamp() as usize;

    let token = encode_jwt(
        Claims {
            res: 0,
            sub: user.user_id,
            rl: user.role,
            exp,
        },
        &state.env.jwt_secret,
    );

    return match token {
        Some(token) => AppResult::Result(StatusCode::CREATED, LogedIn { token }),
        None => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong!"),
        ),
    };
}

pub async fn login_restaurant_owner(
    State(state): State<Arc<AppState>>,
    Json(login_dto): Json<LogIn>,
) -> AppResult<LogedIn> {
    let res = sqlx::query_as!(
        UserModel,
        "SELECT email,password_hash,user_id,role,email_validated from users where email=$1 AND role IN ('RestaurantOwner','Admin')",
        login_dto.email
    )
    .fetch_one(&state.db)
    .await;

    if let Err(_) = res {
        return AppResult::Error(StatusCode::UNAUTHORIZED, String::from("Unauthorized!"));
    }

    let user = res.unwrap();

    if unix::verify(login_dto.password, &user.password_hash) == false {
        return AppResult::Error(StatusCode::UNAUTHORIZED, String::from("Unauthorized!"));
    }

    let now = chrono::Utc::now();
    let minutes: i64 = state.env.jwt_expires_in.parse().unwrap();
    let exp = (now + chrono::Duration::minutes(minutes)).timestamp() as usize;

    let token = encode_jwt(
        Claims {
            res: 0,
            sub: user.user_id,
            rl: user.role,
            exp,
        },
        &state.env.jwt_secret,
    );

    return match token {
        Some(token) => AppResult::Result(StatusCode::CREATED, LogedIn { token }),
        None => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong!"),
        ),
    };
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(register_user_dto): Json<RegisterUser>,
) -> AppResult<LogedIn> {
    let res = sqlx::query_as!(
        User,
        "SELECT user_id, name, last_name, email, role, status ,email_validated
        FROM users
        WHERE email = $1
        AND
        role = 'User'
        LIMIT 1",
        register_user_dto.email
    )
    .fetch_one(&state.db)
    .await;

    if let Ok(user) = res {
        return AppResult::Error(
            StatusCode::CONFLICT,
            format!("{} already exist!", user.email),
        );
    }

    let password_hash = bcrypt::hash(register_user_dto.password).unwrap();
    let res = sqlx::query_as!(
        User,
        "INSERT INTO users (name,last_name,email,password_hash,role) VALUES ($1,$2,$3,$4,$5) RETURNING user_id,name,last_name,email,role,email_validated,status",
        register_user_dto.name,
        register_user_dto.last_name,
        register_user_dto.email,
        password_hash,
        "User"
    )
    .fetch_one(&state.db)
    .await;

    let user = res.unwrap();

    let now = chrono::Utc::now();
    let minutes: i64 = state.env.jwt_expires_in.parse().unwrap();
    let exp = (now + chrono::Duration::minutes(minutes)).timestamp() as usize;

    let token = encode_jwt(
        Claims {
            res: 0,
            sub: user.user_id,
            rl: user.role,
            exp,
        },
        &state.env.jwt_secret,
    );

    return match token {
        Some(token) => AppResult::Result(StatusCode::CREATED, LogedIn { token }),
        None => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong!"),
        ),
    };
}
pub async fn register_restaurant_owner(
    State(state): State<Arc<AppState>>,
    Json(register_user_dto): Json<RegisterUser>,
) -> AppResult<LogedIn> {
    let res = sqlx::query_as!(
        User,
        "SELECT user_id, name, last_name, email, role, email_validated,status
        FROM users
        WHERE email = $1
        AND
        role = 'RestaurantOwner'
        LIMIT 1",
        register_user_dto.email
    )
    .fetch_one(&state.db)
    .await;

    if let Ok(user) = res {
        return AppResult::Error(
            StatusCode::CONFLICT,
            format!("{} already exist!", user.email),
        );
    }

    let password_hash = bcrypt::hash(register_user_dto.password).unwrap();
    let res = sqlx::query_as!(
        User,
        "INSERT INTO users (name,last_name,email,password_hash,role) VALUES ($1,$2,$3,$4,$5) RETURNING user_id,name,last_name,email,role,email_validated,status",
        register_user_dto.name,
        register_user_dto.last_name,
        register_user_dto.email,
        password_hash,
        "RestaurantOwner"
    )
    .fetch_one(&state.db)
    .await;

    let user = res.unwrap();

    let now = chrono::Utc::now();
    let minutes: i64 = state.env.jwt_expires_in.parse().unwrap();
    let exp = (now + chrono::Duration::minutes(minutes)).timestamp() as usize;

    let token = encode_jwt(
        Claims {
            res: 0,
            sub: user.user_id,
            rl: user.role,
            exp,
        },
        &state.env.jwt_secret,
    );

    return match token {
        Some(token) => AppResult::Result(StatusCode::CREATED, LogedIn { token }),
        None => AppResult::Error(
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Something went wrong!"),
        ),
    };
}
