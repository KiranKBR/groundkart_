-- order/up.sql
CREATE TABLE salesorder (
  id SERIAL PRIMARY KEY,
  price FLOAT,
  customer_id INTEGER REFERENCES customer(id)
);
