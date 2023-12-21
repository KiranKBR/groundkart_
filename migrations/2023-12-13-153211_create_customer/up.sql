-- Your SQL goes here
CREATE TABLE customer (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL,
  name TEXT NOT NULL,
  password TEXT NOT NULL,
  phone TEXT NOT NULL,
  cart_id INTEGER REFERENCES cart(id)
);