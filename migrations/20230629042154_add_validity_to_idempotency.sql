-- migrations/{}_add_validity_to_idempotency.sql
ALTER TABLE idempotency ADD COLUMN validity BOOLEAN DEFAULT true;