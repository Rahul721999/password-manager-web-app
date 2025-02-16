-- Add down migration script here
-- To delete the existing table
DROP INDEX IF EXISTS "idx_users_email";
DROP INDEX IF EXISTS "idx_id";
DROP TABLE IF EXISTS "website_credentials";
DROP TABLE IF EXISTS "user_cred";
