-- Your SQL goes here
CREATE TABLE user_disciplines (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    discipline "ClimbType" NOT NULL
);
