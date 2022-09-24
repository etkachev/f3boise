-- Add channel id column for q_line_up
ALTER TABLE q_line_up
    ADD COLUMN channel_id TEXT;

UPDATE q_line_up
SET channel_id = 'C03UR7GM7Q9'
WHERE ao = 'bleach';

UPDATE q_line_up
SET channel_id = 'C03UBFXVBGD'
WHERE ao = 'gem';

UPDATE q_line_up
SET channel_id = 'C03TZTPUFRV'
WHERE ao = 'old-glory';

UPDATE q_line_up
SET channel_id = 'C03V463RFRN'
WHERE ao = 'rebel';

UPDATE q_line_up
SET channel_id = 'C03TZTTHDPZ'
WHERE ao = 'iron-mountain';

UPDATE q_line_up
SET channel_id = 'C03V46DGXMW'
WHERE ao = 'ruckership';

UPDATE q_line_up
SET channel_id = 'C03UEBT1QRZ'
WHERE ao = 'backyard';

UPDATE q_line_up
SET channel_id = 'C03UT46303T',
    ao         = 'rise'
WHERE ao = 'bowler-park'
   OR ao = 'rise';

ALTER TABLE q_line_up
    ALTER COLUMN channel_id SET NOT NULL;

ALTER TABLE q_line_up
    ADD UNIQUE (channel_id, date);
