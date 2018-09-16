-- https://docs.datastax.com/ja/cql-jajp/3.1/cql/cql_reference/create_keyspace_r.html
CREATE KEYSPACE IF NOT EXISTS myapp;

-- https://docs.datastax.com/ja/cql-jajp/3.1/cql/cql_reference/create_table_r.html
CREATE TABLE IF NOT EXISTS myapp.users (
    username text,
    insertion_time timestamp,
    PRIMARY KEY (username)
)
WITH CLUSTERING ORDER BY (insertion_time DESC);

