-- Your SQL goes here
CREATE TABLE "users" (
    id VARCHAR(64) PRIMARY KEY NOT NULL,
    email VARCHAR(256) UNIQUE NOT NULL,
    name VARCHAR(256),
    password_hash VARCHAR(256) NOT NULL
);
