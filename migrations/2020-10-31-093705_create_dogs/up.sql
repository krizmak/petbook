-- Your SQL goes here
CREATE TABLE dogs (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  breed TEXT NOT NULL,
  sex CHAR(1) NOT NULL,
  color TEXT NOT NULL,
  chip_id TEXT,
  description TEXT,
  birth DATE NOT NULL,
  death DATE,
  owner_id INTEGER REFERENCES users(id) NOT NULL,
  address_id INTEGER REFERENCES addresses(id)
);

CREATE TABLE addresses (
  id INTEGER PRIMARY KEY NOT NULL,
  country CHAR(2) NOT NULL,
  state TEXT,
  county TEXT,
  city TEXT,
  postal_code TEXT,
  street TEXT,
  address_line TEXT
)
