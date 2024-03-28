-- New Processed Items table
CREATE TABLE processed_items
(
    id                     uuid      NOT NULL,
    PRIMARY KEY (id),
    item_type              TEXT      NOT NULL,
    item_id                TEXT      NOT NULL,
    initial_date_processed timestamp NOT NULL DEFAULT now(),
    date_updated           timestamp NOT NULL,
    amt_processed          INT       NOT NULL DEFAULT 1
);
