-- Add migration script here
CREATE TABLE items (
    id INT PRIMARY KEY,
    name TEXT,
    category TEXT,
    is_checked BOOL DEFAULT FALSE
)
