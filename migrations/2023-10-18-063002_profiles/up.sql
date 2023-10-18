-- Your SQL goes here
CREATE TABLE public.profiles (
    id uuid NOT NULL,
    username character varying NOT NULL,
    email character varying NOT NULL,
    first_name character varying,
    last_name character varying,
    created_at timestamp(6) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    modified_at timestamp(6) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);

