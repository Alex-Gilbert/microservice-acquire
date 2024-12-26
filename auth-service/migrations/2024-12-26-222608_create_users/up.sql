CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash bytea NOT NULL,
    password_salt bytea NOT NULL
);
