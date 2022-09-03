-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR NOT NULL
);

INSERT INTO
    posts (user_id, title)
VALUES
    (1, 'My first post'),
    (1, 'About Rust'),
    (2, 'My first post too');