-- Your SQL goes here
CREATE TABLE urls (
  id SERIAL PRIMARY KEY,
  original_url TEXT NOT NULL,
  short_url TEXT NOT NULL UNIQUE
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  visits INTEGER NOT NULL DEFAULT 0
);