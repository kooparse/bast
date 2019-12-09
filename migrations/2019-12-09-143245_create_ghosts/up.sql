-- Your SQL goes here

CREATE TABLE ghosts (
  id SERIAL PRIMARY KEY,
  user_id SERIAL references users(id),
  website_id SERIAL references websites(id),
  is_new_session BOOLEAN NOT NULL DEFAULT FALSE,
  pathname TEXT NOT NULL,
  hostname TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);
