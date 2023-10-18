-- Your SQL goes here
CREATE TYPE "ClimbType" AS ENUM (
    'bouldering',
    'top_rope',
    'lead'
);

CREATE TABLE public.climb_location_disciplines (
    id uuid NOT NULL,
    climb_location_id uuid NOT NULL,
    discipline "ClimbType" NOT NULL
);
