-- Add fields for backblasts table
ALTER TABLE back_blasts
    ADD COLUMN title     TEXT,
    ADD COLUMN moleskine TEXT,
    ADD COLUMN fngs      TEXT;