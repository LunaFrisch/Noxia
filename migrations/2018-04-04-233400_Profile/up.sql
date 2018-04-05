-- Your SQL goes here

CREATE TABLE Profiles (
    UserId INTEGER PRIMARY KEY,
    Name TEXT NOT NULL,
    Role TEXT NOT NULL,
    Favor INTEGER NOT NULL,
    TotalClients INTEGER NOT NULL,
    TotalVisits INTEGER NOT NULL
);