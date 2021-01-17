-- Your SQL goes here
CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  informal_name TEXT,
  title VARCHAR(10),
  email TEXT NOT NULL UNIQUE,
  address_id INTEGER REFERENCES addresses(id),
  phone VARCHAR(50),
  password_hash TEXT,
  google_id TEXT,
  facebook_id TEXT,
  disabled BOOLEAN
)
