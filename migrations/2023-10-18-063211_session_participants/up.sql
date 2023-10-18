-- Your SQL goes here
CREATE TABLE public.session_participants (
    id uuid NOT NULL,
    session_id uuid NOT NULL,
    user_id uuid NOT NULL
);
