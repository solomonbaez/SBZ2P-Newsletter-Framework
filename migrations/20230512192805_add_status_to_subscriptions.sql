-- migrations/{timestamp}_add_status_to_subscriptions.sql

ALTER TABLE subscriptions ADD COLUMN status TEXT NULL;