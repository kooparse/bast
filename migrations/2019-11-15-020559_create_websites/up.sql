-- Your SQL goes here

CREATE TABLE websites (
  id SERIAL PRIMARY KEY,
  user_id SERIAL references users(id),
  domain TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);
