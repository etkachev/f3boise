-- Set slack_id column on users table to not null
ALTER TABLE users
    ALTER COLUMN slack_id SET NOT NULL;
