-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS restaurant_menu_items (
        restaurant_menu_item_id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        price FLOAT NOT NULL,
        description TEXT NOT NULL,
        restaurant_menu_id INTEGER NOT NULL,
        cover_image_uri TEXT NOT NULL,
        FOREIGN KEY (restaurant_menu_id) REFERENCES restaurant_menus (restaurant_menu_id) ON DELETE CASCADE
    );