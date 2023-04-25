-- Add migration script here
UPDATE back_blasts
SET ao = 'the-tower'
WHERE channel_id = 'C04B2DX8CCW';

UPDATE q_line_up
SET ao = 'the-tower'
WHERE channel_id = 'C04B2DX8CCW';

UPDATE ao_list
SET name = 'the-tower'
WHERE channel_id = 'C04B2DX8CCW';