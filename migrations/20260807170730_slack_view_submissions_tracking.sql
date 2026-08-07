-- Track processed Slack view submissions to prevent duplicates from webhook retries
-- Each Slack modal submission has a unique view.id that persists across retries

CREATE TABLE IF NOT EXISTS slack_view_submissions (
    view_id TEXT PRIMARY KEY,
    result_id TEXT NOT NULL,
    view_type TEXT NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    -- Automatically clean up old entries after 1 hour
    expires_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() + INTERVAL '1 hour'
);

-- Index for cleanup query
CREATE INDEX idx_slack_view_submissions_expires ON slack_view_submissions(expires_at);

-- Cleanup old entries (run periodically or via trigger)
-- DELETE FROM slack_view_submissions WHERE expires_at < NOW();
