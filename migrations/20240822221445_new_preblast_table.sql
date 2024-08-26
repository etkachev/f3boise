-- Create pre-blasts table
CREATE TABLE pre_blasts
(
    id          uuid NOT NULL,
    PRIMARY KEY (id),
    ao          TEXT NOT NULL,
    channel_id  TEXT NOT NULL,
    title       TEXT NOT NULL,
    qs          TEXT NOT NULL,
    date        DATE NOT NULL,
    start_time  TIME NOT NULL,
    why         TEXT NOT NULL,
    equipment   TEXT,
    fng_message TEXT,
    mole_skin   TEXT,
    img_ids     TEXT,
    ts          TEXT
);
