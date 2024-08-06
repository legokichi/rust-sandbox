PRAGMA foreign_keys = true;

CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE github (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL UNIQUE,
    github_id INTEGER NOT NULL UNIQUE,
    -- access_token TEXT NOT NULL,
    login TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) references users(id) ON DELETE CASCADE
);

CREATE TABLE facebook (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL UNIQUE,
    facebook_id INTEGER NOT NULL UNIQUE,
    -- access_token TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE instagram (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL UNIQUE,
    instagram_id INTEGER NOT NULL UNIQUE,
    -- access_token TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS points (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    text TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE rivers (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    waypoint_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) references users(id) ON DELETE CASCADE,
    FOREIGN KEY (waypoint_id) references waypoints(id) ON DELETE CASCADE
);

CREATE TABLE waypoints (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    elevation REAL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

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