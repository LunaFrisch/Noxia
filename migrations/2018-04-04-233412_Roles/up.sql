-- Your SQL goes here

CREATE TABLE Roles (
    RoleName TEXT NOT NULL,
    tip BOOLEAN NOT NULL,
    clockin BOOLEAN NOT NULL,
    clockout BOOLEAN NOT NULL,
    confirm BOOLEAN NOT NULL,
    deny BOOLEAN NOT NULL,
    list BOOLEAN NOT NULL,
    offer BOOLEAN NOT NULL,
    feedback BOOLEAN NOT NULL,
    gelbooru BOOLEAN NOT NULL,
    rule34 BOOLEAN NOT NULL,
    e621 BOOLEAN NOT NULL,
    yandere BOOLEAN NOT NULL,
    urban BOOLEAN NOT NULL,
    lmgtfy BOOLEAN NOT NULL,
    checkfavor BOOLEAN NOT NULL,
    setrole BOOLEAN NOT NULL,
    setworker BOOLEAN NOT NULL,
    removeworker BOOLEAN NOT NULL,
    blacklist BOOLEAN NOT NULL,
    say BOOLEAN NOT NULL,
    checkfeedback BOOLEAN NOT NULL,
    setpackage BOOLEAN NOT NULL,
    profile BOOLEAN NOT NULL
);