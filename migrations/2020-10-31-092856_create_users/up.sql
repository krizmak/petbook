-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  age INTEGER,
  password_hash TEXT,
  google_id TEXT,
  facebook_id TEXT
)
