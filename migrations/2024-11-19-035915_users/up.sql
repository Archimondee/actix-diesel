-- Your SQL goes here
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    auth_id TEXT NOT NULL,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (auth_id) REFERENCES auths (id)
);