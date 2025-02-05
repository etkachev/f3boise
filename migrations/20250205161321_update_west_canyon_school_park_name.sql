-- Update name for West Canyon Park ao
UPDATE back_blasts
SET
    ao = 'otb-mallard-park'
WHERE
    channel_id = 'C07A9KYGG9X';


UPDATE q_line_up
SET
    ao = 'otb-mallard-park'
WHERE
    channel_id = 'C07A9KYGG9X';


UPDATE ao_list
SET
    name = 'otb-mallard-park'
WHERE
    channel_id = 'C07A9KYGG9X';


UPDATE pre_blasts
SET
    ao = 'otb-mallard-park'
WHERE
    channel_id = 'C07A9KYGG9X';
