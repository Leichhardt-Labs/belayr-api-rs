-- Your SQL goes here
CREATE TABLE public.preferred_locations (
    id uuid NOT NULL,
    user_id uuid NOT NULL,
    climb_location_id uuid NOT NULL
);
