-- product/up.sql
CREATE TABLE product (
  id SERIAL PRIMARY KEY,
  category TEXT NOT NULL,
  name TEXT NOT NULL,
  unit_stock INT
);
