-- Add parent type column for users
ALTER TABLE users
    ADD COLUMN parent_type TEXT;