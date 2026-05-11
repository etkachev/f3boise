-- Add migration script here
UPDATE back_blasts
SET
    ao = 'sunday-ruck'
WHERE
    channel_id = 'C0ATRN16E2U';


UPDATE q_line_up
SET
    ao = 'sunday-ruck'
WHERE
    channel_id = 'C0ATRN16E2U';


UPDATE ao_list
SET
    name = 'sunday-ruck'
WHERE
    channel_id = 'C0ATRN16E2U';


UPDATE pre_blasts
SET
    ao = 'sunday-ruck'
WHERE
    channel_id = 'C0ATRN16E2U';
