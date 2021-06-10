-- Your SQL goes here
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL,
    body VARCHAR NOT NULL
);

INSERT INTO
    comments (post_id, body)
VALUES
    (1, 'Great post'),
    (2, 'Yay! I am learning Rust'),
    (3, 'I enjoyed your post');