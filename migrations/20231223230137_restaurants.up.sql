-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS restaurants (
        restaurant_id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        user_id INTEGER NOT NULL,
        location TEXT NOT NULL,
        cover_image_uri TEXT NOT NULL,
        email TEXT NOT NULL,
        phone TEXT NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
    );