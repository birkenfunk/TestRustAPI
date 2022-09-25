CREATE TABLE users(
                     id INTEGER AUTO_INCREMENT PRIMARY KEY,
                     name VARCHAR(30) NOT NULL UNIQUE ,
                     password VARCHAR(30) NOT NULL,
                     deactivated BOOLEAN DEFAULT FALSE NOT NULL
);
--  Auto-generated SQL script #202209162017
INSERT INTO users (name,password)
VALUES ('Alex','123');

-- Your SQL goes here