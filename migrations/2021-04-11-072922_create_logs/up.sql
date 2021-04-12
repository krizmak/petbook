-- Your SQL goes here
CREATE TABLE logs (
  id INTEGER PRIMARY KEY NOT NULL,
  log_date DATE NOT NULL,
  summary TEXT NOT NULL,
  description TEXT,
  dog_id INTEGER REFERENCES dog(id) NOT NULL
);
