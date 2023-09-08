-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username VARCHAR NOT NULL,
    password_hash VARCHAR NOT NULL,
    salt VARCHAR NOT NULL,
    priviledge INT NOT NULL
)