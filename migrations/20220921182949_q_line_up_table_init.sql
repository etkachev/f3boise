-- Create initial q line up table
CREATE TABLE q_line_up
(
    id          uuid      NOT NULL,
    PRIMARY KEY (id),
    qs          TEXT      NOT NULL,
    ao          TEXT      NOT NULL,
    date        DATE      NOT NULL,
    closed      BOOLEAN   NOT NULL,
    create_date timestamp NOT NULL DEFAULT now()
);

