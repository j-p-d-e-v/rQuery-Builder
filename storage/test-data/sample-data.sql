INSERT INTO users (name, email) VALUES
('Alice', 'alice@example.com'),
('Bob', 'bob@example.com'),
('Carol', 'carol@example.com');

INSERT INTO products (name, price) VALUES
('Laptop', 1200.00),
('Phone', 800.00),
('Tablet', 400.00);

INSERT INTO orders (user_id, product_id, quantity, order_date) VALUES
(1, 1, 1, '2025-07-01'),  -- Alice orders Laptop
(1, 2, 2, '2025-07-02'),  -- Alice orders 2 Phones
(2, 3, 1, '2025-07-03');  -- Bob orders Tablet
-- Note: Carol (id=3) has no orders (useful for LEFT JOIN testing)

