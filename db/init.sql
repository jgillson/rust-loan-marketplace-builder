CREATE TABLE events
(
    aggregate_type text                         NOT NULL,
    aggregate_id   text                         NOT NULL,
    sequence       bigint CHECK (sequence >= 0) NOT NULL,
    payload        text                         NOT NULL,
    metadata       text                         NOT NULL,
    timestamp      timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);

CREATE TABLE group_query
(
    query_instance_id text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           text                        NOT NULL,
    PRIMARY KEY (query_instance_id)
);

CREATE USER test_user WITH ENCRYPTED PASSWORD 'test_pass';
GRANT ALL PRIVILEGES ON DATABASE postgres TO test_user;
