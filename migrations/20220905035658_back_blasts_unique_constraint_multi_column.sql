-- Unique constraint on multi columns for back blasts
ALTER TABLE back_blasts
    ADD UNIQUE (ao, date, bb_type);
