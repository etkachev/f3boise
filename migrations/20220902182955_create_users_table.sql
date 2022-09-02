-- Create Users Table
CREATE TABLE users
(
    id          uuid      NOT NULL,
    PRIMARY KEY (id),
    slack_id    TEXT      NULL,
    name        TEXT      NOT NULL,
    email       TEXT      NOT NULL,
    create_date timestamp NOT NULL DEFAULT now()
);
