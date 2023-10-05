-- update name for camels back
-- Add migration script here
UPDATE back_blasts
SET ao = 'camels-back'
WHERE channel_id = 'C05AJDFUBM4';

UPDATE q_line_up
SET ao = 'camels-back'
WHERE channel_id = 'C05AJDFUBM4';

UPDATE ao_list
SET name = 'camels-back'
WHERE channel_id = 'C05AJDFUBM4';