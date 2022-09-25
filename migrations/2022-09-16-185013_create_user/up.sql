CREATE TABLE users(
                     user_id INTEGER AUTO_INCREMENT PRIMARY KEY,
                     user_name VARCHAR(30) NOT NULL UNIQUE ,
                     user_password VARCHAR(30) NOT NULL,
                     account_deactivated BOOLEAN DEFAULT FALSE NOT NULL
);
--  Auto-generated SQL script #202209162017
INSERT INTO users (user_name,user_password)
VALUES ('Alex','123');

-- Your SQL goes here