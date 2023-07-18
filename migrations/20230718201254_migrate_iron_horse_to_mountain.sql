-- migrate all BDs from iron-horse to iron-mountain
-- iron-horse:  C058WNHF24A
-- iron-mountain: C03TZTTHDPZ

UPDATE back_blasts
SET ao         = 'iron-mountain',
    channel_id = 'C03TZTTHDPZ'
WHERE channel_id = 'C058WNHF24A';

UPDATE q_line_up
SET ao         = 'iron-mountain',
    channel_id = 'C03TZTTHDPZ'
WHERE channel_id = 'C058WNHF24A';