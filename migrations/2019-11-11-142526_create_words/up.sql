-- Your SQL goes here

CREATE TABLE words (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    curr TEXT NOT NULL,
    next TEXT,
    start_sentinel BOOLEAN NOT NULL DEFAULT FALSE,
    end_sentinel BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX ON words (curr);
CREATE INDEX ON words (start_sentinel);