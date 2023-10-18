-- Your SQL goes here
CREATE TYPE "VenueType" AS ENUM (
    'indoor',
    'outdoor'
);

CREATE TABLE climb_locations (
    id uuid PRIMARY KEY,
    name character varying NOT NULL,
    address_line_one character varying,
    address_line_two character varying,
    city character varying NOT NULL,
    state character varying NOT NULL,
    postal_code character varying NOT NULL,
    country character varying NOT NULL
);


