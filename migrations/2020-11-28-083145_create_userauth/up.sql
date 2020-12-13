-- Your SQL goes here
CREATE TABLE userauth (
  id INTEGER PRIMARY KEY NOT NULL,
  user_id INTEGER NOT NULL,
  password_hash TEXT,
  google_id TEXT,
  facebook_id TEXT,
  FOREIGN KEY(user_id) REFERENCES users(id)
)
