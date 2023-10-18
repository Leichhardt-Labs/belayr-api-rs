-- Your SQL goes here
CREATE TABLE friends (
    id uuid PRIMARY KEY,
    profile_one uuid NOT NULL,
    profile_two uuid NOT NULL,
    is_accepted boolean,
    created_at timestamp(6) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT friends_profile_one_profile_two_idx UNIQUE (profile_one, profile_two)
);
