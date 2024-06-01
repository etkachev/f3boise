-- Add parent column for users
ALTER TABLE users
    ADD COLUMN parent TEXT;