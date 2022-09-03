-- Your SQL goes here
CREATE TABLE animals (
    id SERIAL PRIMARY KEY,
    species VARCHAR NOT NULL,
    legs INTEGER NOT NULL,
    name VARCHAR
);

INSERT INTO
    animals (species, legs, name)
VALUES
    ('dog', 4, 'Jack'),
    ('spider', 8, null);