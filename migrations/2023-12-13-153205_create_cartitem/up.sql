-- Your SQL goes here
CREATE TABLE cartitem (
  id SERIAL PRIMARY KEY,
  quantity INT,
  product_id INTEGER REFERENCES product(id),
  cart_id INTEGER REFERENCES cart(id)
);