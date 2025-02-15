ALTER TABLE user_cred DROP COLUMN auth_provider;
ALTER TABLE user_cred DROP COLUMN google_id;
DROP INDEX IF EXISTS idx_user_cred_google_id;
DROP TYPE IF EXISTS auth_provider_enum;

-- Restore password_hash as NOT NULL
ALTER TABLE user_cred ALTER COLUMN password_hash SET NOT NULL;
