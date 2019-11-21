-- Your SQL goes here

CREATE TABLE websites (
  id SERIAL PRIMARY KEY,
  user_id SERIAL references users(id),
  visitors INTEGER NOT NULL DEFAULT 0,
  sessions INTEGER NOT NULL DEFAULT 0,
  domain TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);

CREATE TABLE pages (
  id SERIAL PRIMARY KEY,
  website_id SERIAL references websites(id),
  pathname TEXT NOT NULL UNIQUE,
  visitors INTEGER NOT NULL DEFAULT 0,
  sessions INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);
