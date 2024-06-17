-- Add unique constraint
ALTER TABLE parent_pax_relationships
    ADD CONSTRAINT unique_pax_name UNIQUE (pax_name);