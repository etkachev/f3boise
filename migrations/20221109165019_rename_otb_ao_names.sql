-- Update otb AO names

UPDATE back_blasts
SET ao = 'warhorse'
WHERE channel_id = 'C0425DL9MT7';

UPDATE back_blasts
SET ao = 'bellagio'
WHERE channel_id = 'C045SMRL43X';

UPDATE q_line_up
SET ao = 'warhorse'
WHERE channel_id = 'C0425DL9MT7';

UPDATE q_line_up
SET ao = 'bellagio'
WHERE channel_id = 'C045SMRL43X';
