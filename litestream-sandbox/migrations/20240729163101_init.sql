PRAGMA foreign_keys = true;

CREATE TABLE roles (
    role_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    role_name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE users (
    user_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    role_id INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (role_id) REFERENCES roles(role_id) ON DELETE CASCADE
);

CREATE TABLE github (
    github_id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE,
    login TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE TABLE facebook (
    facebook_id INTEGER NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

INSERT INTO roles (role_name) VALUES ('default');
INSERT INTO roles (role_name) VALUES ('admin');
INSERT INTO users (role_id) VALUES (2); -- admin user
INSERT INTO github (github_id, user_id, login) VALUES (2429307, 1, 'legokichi');

CREATE TABLE access_logs (
    access_log_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    request TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);
