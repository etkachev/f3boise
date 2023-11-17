-- Update ruckership west to just regular for now
UPDATE back_blasts
SET ao = 'ruckership'
WHERE channel_id = 'C03V46DGXMW';

UPDATE q_line_up
SET ao = 'ruckership'
WHERE channel_id = 'C03V46DGXMW';

UPDATE ao_list
SET name = 'ruckership'
WHERE channel_id = 'C03V46DGXMW';