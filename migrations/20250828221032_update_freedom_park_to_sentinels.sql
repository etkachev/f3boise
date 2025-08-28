-- Update freedom park to sentinels
UPDATE back_blasts
SET
    ao = 'sentinels'
WHERE
    channel_id = 'C08QR6U5W2V';


UPDATE q_line_up
SET
    ao = 'sentinels'
WHERE
    channel_id = 'C08QR6U5W2V';


UPDATE ao_list
SET
    name = 'sentinels'
WHERE
    channel_id = 'C08QR6U5W2V';


UPDATE pre_blasts
SET
    ao = 'sentinels'
WHERE
    channel_id = 'C08QR6U5W2V';
