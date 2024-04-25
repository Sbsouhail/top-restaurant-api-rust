-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS restaurant_menus (
        restaurant_menu_id SERIAL PRIMARY KEY,
        restaurant_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        is_active BOOLEAN NOT NULL DEFAULT false,
        FOREIGN KEY (restaurant_id) REFERENCES restaurants (restaurant_id) ON DELETE CASCADE
    );