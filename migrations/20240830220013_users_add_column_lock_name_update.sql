-- Add new column to users for locked_name_update
ALTER TABLE users
    ADD COLUMN locked_name_update BOOLEAN;

UPDATE users
-- set default to locked
SET locked_name_update = true
WHERE locked_name_update IS NULL;


ALTER TABLE users
    ALTER COLUMN locked_name_update SET NOT NULL;
