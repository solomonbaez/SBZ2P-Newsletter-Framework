-- migrations/{}_remove_salt_from_users.sql
ALTER TABLE users DROP COLUMN salt;