use std::sync::Arc;

use axum::{
    extract::DefaultBodyLimit,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware,
    routing::{delete, get, post},
    Router,
};

use common::{auth_middleware::auth_middleware, role_middleware::role_middleware};
use config::Config;
use dotenv::dotenv;
use modules::{
    auth::auth_controller::{
        login, login_restaurant_owner, register_restaurant_owner, register_user,
    },
    files::files_controller::upload,
    restaurants::restaurants_controller::{
        create_restaurant, delete_restaurant, get_my_restaurants, get_restaurant, get_restaurants,
    },
    users::{
        users_controller::{accept_restaurant_owner, block_restaurant_owner, get_me, get_users},
        users_dto::{RolesEnum, User},
    },
};
use pwhash::bcrypt;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

mod common;
mod config;
mod modules;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let shared_state = Arc::new(AppState {
        db: pool,
        env: config,
    });

    let password_hash = bcrypt::hash("Test123#").unwrap();

    let res = sqlx::query_as!(
        User,
        "INSERT INTO users (name,last_name,email,password_hash,role) VALUES ($1,$2,$3,$4,$5) RETURNING user_id,name,last_name,email,role,status,email_validated",
        "Souhail",
        "SBOUI",
        "sbsouhail@gmail.com",
        password_hash,
        "Admin"
    )
    .fetch_one(&shared_state.db)
    .await;

    match res {
        Ok(_) => println!("Admin seeded"),
        Err(_) => println!("Admin seed failed"),
    };

    let restaurants_routes = Router::new()
        .route("/", post(create_restaurant))
        .route("/me", get(get_my_restaurants))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            |state, req, next| role_middleware(state, req, next, RolesEnum::RestaurantOwner),
        ))
        .route("/:restaurant_id", delete(delete_restaurant))
        .layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth_middleware,
        ))
        .route("/:restaurant_id", get(get_restaurant))
        .route("/", get(get_restaurants));

    let users_routes = Router::new()
        .route("/", get(get_users))
        .route(
            "/restaurant-owner/:user_id/accept",
            post(accept_restaurant_owner),
        )
        .route(
            "/restaurant-owner/:user_id/reject",
            post(block_restaurant_owner),
        )
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            |state, req, next| role_middleware(state, req, next, RolesEnum::Admin),
        ))
        .route_layer(middleware::from_fn_with_state(
            shared_state.clone(),
            |state, req, next| role_middleware(state, req, next, RolesEnum::User),
        ))
        .route("/me", get(get_me))
        .layer(middleware::from_fn_with_state(
            shared_state.clone(),
            auth_middleware,
        ));

    let auth_routes = Router::new()
        .route("/login", post(login))
        .route("/login/restaurant-owner", post(login_restaurant_owner))
        .route("/register", post(register_user))
        .route(
            "/register-restaurant-onwner",
            post(register_restaurant_owner),
        );

    let files_routes = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/upload", post(upload))
        .layer(DefaultBodyLimit::max(10000000));

    let api_routes = Router::new()
        .nest("/restaurants", restaurants_routes)
        .nest("/users", users_routes)
        .nest("/auth", auth_routes)
        .nest("/files", files_routes);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = Router::new()
        .nest("/api", api_routes)
        .with_state(shared_state.clone())
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
