-- Your SQL goes here
CREATE TABLE session_participants (
    id uuid PRIMARY KEY,
    session_id uuid NOT NULL,
    user_id uuid NOT NULL
);
