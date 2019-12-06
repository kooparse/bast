-- Your SQL goes here

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  password  VARCHAR(64) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);

ALTER TABLE users
  ADD CONSTRAINT password_chk CHECK (char_length(password) >= 6);
