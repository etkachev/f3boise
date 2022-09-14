-- Add migration script here
ALTER TABLE back_blasts
    ADD COLUMN channel_id TEXT,
    ADD COLUMN active     BOOLEAN;

UPDATE back_blasts
SET channel_id = 'C03UR7GM7Q9',
    active     = true
WHERE ao = 'bleach';

UPDATE back_blasts
SET channel_id = 'C03UBFXVBGD',
    active     = true
WHERE ao = 'gem';

UPDATE back_blasts
SET channel_id = 'C03TZTPUFRV',
    active     = true
WHERE ao = 'old-glory';

UPDATE back_blasts
SET channel_id = 'C03V463RFRN',
    active     = true
WHERE ao = 'rebel';

UPDATE back_blasts
SET channel_id = 'C03TZTTHDPZ',
    active     = true
WHERE ao = 'iron-mountain';

UPDATE back_blasts
SET channel_id = 'C03V46DGXMW',
    active     = true
WHERE ao = 'ruckership';

UPDATE back_blasts
SET channel_id = 'C03UEBT1QRZ',
    active     = true
WHERE ao = 'backyard';

UPDATE back_blasts
SET channel_id = 'C03UT46303T',
    active     = false,
    ao         = 'rise'
WHERE ao = 'bowler-park'
   OR ao = 'rise';

ALTER TABLE back_blasts
    ALTER COLUMN channel_id SET NOT NULL,
    ALTER COLUMN active SET NOT NULL;
