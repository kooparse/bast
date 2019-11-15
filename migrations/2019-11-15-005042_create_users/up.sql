-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(64) NOT NULL UNIQUE,
  password  VARCHAR(64) NOT NULL,
  email TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);

ALTER TABLE users
  ADD CONSTRAINT password_chk CHECK (char_length(password) >= 6);
