-- Unique constraint on multi columns for reactions log
ALTER TABLE reactions_log
    ADD UNIQUE (slack_user, reaction_timestamp);
