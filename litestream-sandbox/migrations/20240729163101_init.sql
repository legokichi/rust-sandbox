PRAGMA foreign_keys = true;

CREATE TABLE roles (
    -- role id
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    -- role name
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE users (
    -- user id
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    role_id INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

CREATE TABLE github (
    -- github id
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE,
    -- access_token TEXT NOT NULL,
    login TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE facebook (
    -- facebook id
    id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE,
    -- access_token TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

INSERT INTO roles (name) VALUES ('default');
INSERT INTO roles (name) VALUES ('admin');
INSERT INTO users (role_id) VALUES (2); -- admin user
INSERT INTO github (id, user_id, login) VALUES (2429307, 1, 'legokichi');

CREATE TABLE access_log (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    request TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);


--CREATE TABLE instagram (
--    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
--    user_id INTEGER NOT NULL UNIQUE,
--    instagram_id INTEGER NOT NULL UNIQUE,
--    -- access_token TEXT NOT NULL,
--    name TEXT NOT NULL,
--    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
--    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
--    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
--);
--
--CREATE TABLE IF NOT EXISTS points (
--    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
--    timestamp TEXT NOT NULL,
--    text TEXT NOT NULL,
--    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
--    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
--);
--
--CREATE TABLE rivers (
--    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
--    user_id INTEGER NOT NULL,
--    waypoint_id INTEGER NOT NULL,
--    name TEXT NOT NULL,
--    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
--    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
--    FOREIGN KEY (user_id) references users(id) ON DELETE CASCADE,
--    FOREIGN KEY (waypoint_id) references waypoints(id) ON DELETE CASCADE
--);
--
--CREATE TABLE waypoints (
--    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
--    latitude REAL NOT NULL,
--    longitude REAL NOT NULL,
--    elevation REAL,
--    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
--    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
--);

-- CREATE TABLE routes (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     name TEXT
-- );

-- CREATE TABLE route_points (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     route_id INTEGER,
--     latitude REAL NOT NULL,
--     longitude REAL NOT NULL,
--     elevation REAL,
--     time TEXT,
--     FOREIGN KEY (route_id) REFERENCES routes (id)
-- );

-- CREATE TABLE tracks (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     name TEXT
-- );

-- CREATE TABLE track_segments (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     track_id INTEGER,
--     FOREIGN KEY (track_id) REFERENCES tracks (id)
-- );

-- CREATE TABLE track_points (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     segment_id INTEGER,
--     latitude REAL NOT NULL,
--     longitude REAL NOT NULL,
--     elevation REAL,
--     time TEXT,
--     FOREIGN KEY (segment_id) REFERENCES track_segments (id)
-- );
