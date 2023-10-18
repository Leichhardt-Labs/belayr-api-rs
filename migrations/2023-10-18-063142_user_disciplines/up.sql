-- Your SQL goes here
CREATE TABLE public.user_disciplines (
    id uuid NOT NULL,
    user_id uuid NOT NULL,
    discipline public."ClimbType"
);
