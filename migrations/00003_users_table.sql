CREATE TABLE users (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    phone VARCHAR(15) NULL,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL DEFAULT extract(epoch from now()),
    updated_at BIGINT NOT NULL DEFAULT extract(epoch from now()),
    UNIQUE(email),
    UNIQUE(username),
    PRIMARY KEY (id)
);