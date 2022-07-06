CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    role VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    api_key VARCHAR NOT NULL
);

INSERT INTO users(name, role, email, password, api_key) VALUES 
('Superadmin 1', 'Superadmin', 'superadmin1@test.com', '$2b$10$uJmFdn3aE5VV7fdM2GwwSOYyIbWvexp6Z0XvshZJ3qrmN141KJpDq', 'd2c2236f9f8361d7043b4c17d3de1de55e75779a26336b1d6b3abfe0027281fc'),
('Admin 1','Admin','admin1@test.com','$2b$10$7Aqohk4M5h7YhE/9/R8zjuqZ5zbPOSaRmzT8PtRJ7PdgvnRTdtknq', 'c332856b12a28daf3b8f7c52db4dd0a75cc4179c33cde15ba4e4a74e9c9224ff'), 
('User 1','User','user1@test.com','$2b$10$ekxD3kTJvnQZ0YRGgMb5lOo.QHCFgO6Hn6kd9TlXeTYAqHCBhTPSy', 'b3333cfd5e7a2a9d707d9f600b79baab5875d8a3e8a550b1955b40c6a5d0cdd1');