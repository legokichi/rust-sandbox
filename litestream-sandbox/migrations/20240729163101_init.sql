CREATE TABLE IF NOT EXISTS users
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    github_id    INTEGER,
    facebook_id  INTEGER
);

CREATE TABLE IF NOT EXISTS points
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    text      TEXT NOT NULL

);

CREATE TABLE IF NOT EXISTS rivers
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    text      TEXT NOT NULL
);
