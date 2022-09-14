-- Unique constraint on multi columns for back blasts
ALTER TABLE back_blasts
    ADD UNIQUE (channel_id, date, bb_type);
