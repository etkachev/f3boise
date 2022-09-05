-- Create back blasts table
CREATE TABLE back_blasts
(
    id      uuid NOT NULL,
    PRIMARY KEY (id),
    ao      TEXT NOT NULL,
    q       TEXT NOT NULL,
    pax     TEXT NOT NULL,
    date    DATE NOT NULL,
    bb_type TEXT NOT NULL
);
