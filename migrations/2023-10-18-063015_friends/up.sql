-- Your SQL goes here
CREATE TABLE public.friends (
    id uuid NOT NULL,
    profile_one uuid NOT NULL,
    profile_two uuid NOT NULL,
    is_accepted boolean,
    created_at timestamp(6) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);
