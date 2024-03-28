-- Add unique constraint for multi columns on processed_items table
ALTER TABLE processed_items
    ADD UNIQUE (item_type, item_id);