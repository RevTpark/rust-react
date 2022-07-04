CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

INSERT INTO users VALUES (1, 'Superadmin 1', 'Superadmin', 'superadmin1@test.com', 'superadmin1'), 
(2,'Admin 1','Admin','admin1@test.com','admin1'), 
(3,'User 1','User','user1@test.com','user1');