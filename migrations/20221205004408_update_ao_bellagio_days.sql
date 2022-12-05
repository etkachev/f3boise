-- Update Ao bellagio to include saturdays
UPDATE ao_list
SET days = 'Tue,Thu,Sat'
WHERE channel_id = 'C045SMRL43X';
