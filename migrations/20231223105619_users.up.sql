-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS users (
        user_id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        email TEXT UNIQUE NOT NULL,
        password_hash TEXT NOT NULL,
        role TEXT NOT NULL DEFAULT 'User',
        is_restaurant_owner_request BOOLEAN NOT NULL DEFAULT false,
        CHECK (role IN ('User', 'RestaurantOwner', 'Admin'))
    );