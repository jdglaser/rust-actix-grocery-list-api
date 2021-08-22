-- Add migration script here
CREATE TABLE item (
    item_id INTEGER PRIMARY KEY,
    name TEXT,
    category TEXT,
    is_checked BOOL DEFAULT FALSE,
    CREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE user (
    user_id INTEGER PRIMARY KEY,
    username TEXT,
    password TEXT,
    CREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);