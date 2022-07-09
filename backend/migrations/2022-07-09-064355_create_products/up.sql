CREATE TABLE products(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    price INTEGER NOT NULL,
    description VARCHAR NOT NULL,
    created_by INTEGER NOT NULL,
    CONSTRAINT fk_user  
    FOREIGN KEY(created_by)   
    REFERENCES users(id)
);

INSERT INTO products(name, price, description, created_by) VALUES 
('Product 1', 200, 'This is Product 1', 2),
('Product 2', 400, 'This is Product 2', 3),
('Product 3', 100, 'This is Product 3', 3),
('Product 4', 300, 'This is Product 4', 2),
('Product 5', 350, 'This is Product 5', 3);