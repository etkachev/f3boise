-- Create AO List table
CREATE TABLE ao_list
(
    id   uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    days TEXT NOT NULL
);
