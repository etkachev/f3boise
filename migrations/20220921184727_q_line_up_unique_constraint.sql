-- Unique constraint on multi columns for q line up
ALTER TABLE q_line_up
    ADD UNIQUE (ao, date);
