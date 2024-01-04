-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS users (
        user_id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        email TEXT NOT NULL,
        password_hash TEXT NOT NULL,
        role TEXT NOT NULL DEFAULT 'User',
        email_validated BOOLEAN NOT NULL DEFAULT false,
        CHECK (role IN ('User', 'RestaurantOwner', 'Admin')),
        status TEXT NOT NULL DEFAULT 'Pending',
        CHECK (status IN ('Pending', 'Accepted', 'Blocked')),
        UNIQUE (role, email)
    );