-- Make slack_id column in users table unique
ALTER TABLE users
    ADD CONSTRAINT unique_slack_id UNIQUE (slack_id);
