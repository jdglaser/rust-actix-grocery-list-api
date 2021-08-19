-- Add migration script here
CREATE TABLE items (
    id INTEGER PRIMARY KEY,
    name TEXT,
    category TEXT,
    is_checked BOOL DEFAULT FALSE
);

INSERT INTO items (name, category)
VALUES ('foo', 'cat1'), ('bar', 'cat2');