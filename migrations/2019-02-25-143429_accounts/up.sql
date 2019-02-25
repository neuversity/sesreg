CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    nama TEXT NOT NULL,
    email TEXT NOT NULL,
    alamat TEXT NOT NULL DEFAULT ''
);
