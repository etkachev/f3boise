-- Create Parent Pax relationship table
CREATE TABLE parent_pax_relationships
(
    id       uuid NOT NULL,
    PRIMARY KEY (id),
    pax_name TEXT NOT NULL,
    slack_id TEXT NULL,
    parent   JSON NOT NULL
);