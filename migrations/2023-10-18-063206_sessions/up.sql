-- Your SQL goes here
CREATE TABLE sessions (
    id uuid PRIMARY KEY,
    name character varying NOT NULL,
    location_id uuid NOT NULL,
    start_time timestamp(6) without time zone NOT NULL,
    end_time timestamp(6) without time zone,
    is_public boolean NOT NULL,
    discipline "ClimbType" NOT NULL,
    author_id uuid NOT NULL,
    maximum_participants integer
);
