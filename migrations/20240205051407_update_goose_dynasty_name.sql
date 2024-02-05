-- Update molenaar park to goose-dynasty
UPDATE back_blasts
SET ao = 'goose-dynasty'
WHERE channel_id = 'C06DP3D5VTK';

UPDATE q_line_up
SET ao = 'goose-dynasty'
WHERE channel_id = 'C06DP3D5VTK';

UPDATE ao_list
SET name = 'goose-dynasty'
WHERE channel_id = 'C06DP3D5VTK';