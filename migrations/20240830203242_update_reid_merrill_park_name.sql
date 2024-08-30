-- Update name for Reid Merrill Park
UPDATE back_blasts
SET ao = 'coop'
WHERE channel_id = 'C05UUDKULGY';

UPDATE q_line_up
SET ao = 'coop'
WHERE channel_id = 'C05UUDKULGY';

UPDATE ao_list
SET name = 'coop'
WHERE channel_id = 'C05UUDKULGY';

UPDATE pre_blasts
SET ao = 'coop'
WHERE channel_id = 'C05UUDKULGY';
