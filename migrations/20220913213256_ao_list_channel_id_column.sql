-- Add migration script here
ALTER TABLE ao_list
    ADD COLUMN channel_id TEXT,
    ADD COLUMN active     BOOLEAN;

UPDATE ao_list
SET channel_id = 'C03UR7GM7Q9',
    active     = true
WHERE name = 'bleach';

UPDATE ao_list
SET channel_id = 'C03UBFXVBGD',
    active     = true
WHERE name = 'gem';

UPDATE ao_list
SET channel_id = 'C03TZTPUFRV',
    active     = true
WHERE name = 'old-glory';

UPDATE ao_list
SET channel_id = 'C03V463RFRN',
    active     = true
WHERE name = 'rebel';

UPDATE ao_list
SET channel_id = 'C03TZTTHDPZ',
    active     = true
WHERE name = 'iron-mountain';

UPDATE ao_list
SET channel_id = 'C03V46DGXMW',
    active     = true
WHERE name = 'ruckership';

UPDATE ao_list
SET channel_id = 'C03UEBT1QRZ',
    active     = true
WHERE name = 'backyard';

UPDATE ao_list
SET channel_id = 'C03UT46303T',
    active     = false
WHERE name = 'bowler-park'
   OR name = 'rise';

ALTER TABLE ao_list
    ALTER COLUMN channel_id SET NOT NULL,
    ALTER COLUMN active SET NOT NULL;


