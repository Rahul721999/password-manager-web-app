CREATE TABLE IF NOT EXISTS "user_cred" (
    id UUID NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL
);
CREATE INDEX idx_users_email ON user_cred (email);

CREATE TABLE IF NOT EXISTS "website_credentials" (
    id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES user_cred(id),
    website_name TEXT NOT NULL,
    website_url TEXT NOT NULL,
    username TEXT NOT NULL,
    password_hash BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_id ON website_credentials(id);