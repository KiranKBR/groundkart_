-- Your SQL goes here
CREATE TABLE orderitem (
  id SERIAL PRIMARY KEY,
  quantity INT,
  price FLOAT,
  product_id INTEGER REFERENCES product(id),
  salesorder_id INTEGER REFERENCES salesorder(id)
);