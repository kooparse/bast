-- Your SQL goes here

CREATE TABLE ghosts (
  id SERIAL PRIMARY KEY,
  uuid  TEXT NOT NULL,
  puuid SERIAL references ghosts(uuid),
  wid SERIAL references websites(id),
  cid SERIAL references users(id),
  href TEXT NOT NULL,
  pathname TEXT NOT NULL,
  pages TEXT[] NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);
