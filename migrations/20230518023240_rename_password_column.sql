-- migrations/{}_rename_password_column.sql
ALTER TABLE users RENAME password TO password_hash;