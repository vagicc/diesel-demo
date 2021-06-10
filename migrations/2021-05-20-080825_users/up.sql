-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);
INSERT INTO users (name) VALUES ('Sean'), ('Tess');