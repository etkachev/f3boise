-- Add ts column to backblasts
ALTER TABLE back_blasts
    ADD COLUMN ts TEXT;