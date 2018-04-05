-- Your SQL goes here

CREATE TABLE Purchases (
    UserId INTEGER PRIMARY KEY,
    gelbooru BOOLEAN NOT NULL,
    rule34 BOOLEAN NOT NULL,
    e621 BOOLEAN NOT NULL,
    yandere BOOLEAN NOT NULL,
    lmgtfy BOOLEAN NOT NULL
);