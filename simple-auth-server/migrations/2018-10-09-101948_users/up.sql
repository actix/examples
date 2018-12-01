-- Your SQL goes here
CREATE TABLE users (
  email VARCHAR(100) NOT NULL UNIQUE PRIMARY KEY,
  password VARCHAR(64) NOT NULL, --bcrypt hash
  created_at TIMESTAMP NOT NULL
);
