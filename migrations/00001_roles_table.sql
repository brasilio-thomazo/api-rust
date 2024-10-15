CREATE TABLE roles (
    id SERIAL,
    name VARCHAR(50) NOT NULL,
    UNIQUE(name),
    PRIMARY KEY (id)
);