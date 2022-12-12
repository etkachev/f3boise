-- Ruckership is split to east and west

UPDATE back_blasts
SET ao = 'ruckership-west'
WHERE channel_id = 'C03V46DGXMW';

UPDATE q_line_up
SET ao = 'ruckership-west'
WHERE channel_id = 'C03V46DGXMW';

UPDATE ao_list
SET name = 'ruckership-west'
WHERE channel_id = 'C03V46DGXMW';