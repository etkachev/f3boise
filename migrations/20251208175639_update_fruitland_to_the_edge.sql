-- Update fruitland to the edge
UPDATE back_blasts
SET
    ao = 'the-edge'
WHERE
    channel_id = 'C09GCA1QHFB';


UPDATE q_line_up
SET
    ao = 'the-edge'
WHERE
    channel_id = 'C09GCA1QHFB';


UPDATE ao_list
SET
    name = 'the-edge'
WHERE
    channel_id = 'C09GCA1QHFB';


UPDATE pre_blasts
SET
    ao = 'the-edge'
WHERE
    channel_id = 'C09GCA1QHFB';
