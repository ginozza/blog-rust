-- Your SQL goes here

CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  post_id INTEGER NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
  author_name VARCHAR,
  author_email VARCHAR,
  content TEXT NOT NULL,
  created_at TIMESTAMP,
  updated_at TIMESTAMP
);
