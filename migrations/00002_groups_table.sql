CREATE TABLE groups (
    id SERIAL,
    name VARCHAR(50) NOT NULL,
    created_at BIGINT NOT NULL DEFAULT extract(epoch from now()),
    updated_at BIGINT NOT NULL DEFAULT extract(epoch from now()),
    UNIQUE(name),
    PRIMARY KEY (id)
);