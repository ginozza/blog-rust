-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    slug VARCHAR NOT NULL UNIQUE,
    body TEXT NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
