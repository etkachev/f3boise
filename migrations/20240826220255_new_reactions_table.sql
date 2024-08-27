-- Add new reactions log table
CREATE TABLE reactions_log
(
    id                 uuid      NOT NULL,
    PRIMARY KEY (id),
    entity_type        TEXT      NOT NULL,
    entity_id          UUID      NOT NULL,
    reaction           TEXT      NOT NULL,
    slack_user         TEXT      NOT NULL,
    reaction_added     BOOLEAN   NOT NULL,
    reaction_timestamp TIMESTAMP NOT NULL
);