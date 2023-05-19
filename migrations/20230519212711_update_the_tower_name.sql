-- Add migration script here
UPDATE back_blasts
SET ao = 'tower'
WHERE channel_id = 'C04B2DX8CCW';

UPDATE q_line_up
SET ao = 'tower'
WHERE channel_id = 'C04B2DX8CCW';

UPDATE ao_list
SET name = 'tower'
WHERE channel_id = 'C04B2DX8CCW';