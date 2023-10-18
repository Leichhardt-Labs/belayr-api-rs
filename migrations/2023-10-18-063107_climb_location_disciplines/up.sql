-- Your SQL goes here
CREATE TYPE "ClimbType" AS ENUM (
    'bouldering',
    'top_rope',
    'lead'
);

CREATE TABLE climb_location_disciplines (
    id uuid PRIMARY KEY,
    climb_location_id uuid NOT NULL,
    discipline "ClimbType" NOT NULL
);
