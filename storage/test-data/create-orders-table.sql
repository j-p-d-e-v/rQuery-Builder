CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  user_id INT REFERENCES users(id),
  product_id INT REFERENCES products(id),
  quantity INT NOT NULL,
  order_date DATE NOT NULL
);
