-- Make Name column in ao list unique
ALTER TABLE ao_list
    ADD CONSTRAINT unique_name UNIQUE (name);
