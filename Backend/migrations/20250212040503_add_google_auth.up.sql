-- Create ENUM type for authentication providers
CREATE TYPE auth_provider_enum AS ENUM ('email', 'google');

-- create a column using that ENUM type
ALTER TABLE user_cred
ADD COLUMN auth_provider auth_provider_enum NOT NULL DEFAULT 'email';

--- create new column for storing google-id for oAuth signer
ALTER TABLE user_cred
ADD COLUMN google_id TEXT UNIQUE DEFAULT NULL;

CREATE INDEX idx_user_cred_google_id ON user_cred(google_id);

ALTER TABLE user_cred ALTER COLUMN password_hash DROP NOT NULL;
