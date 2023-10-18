-- Your SQL goes here
CREATE TABLE preferred_locations (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    climb_location_id uuid NOT NULL,
    CONSTRAINT preferredLocations_climb_location_id_user_id_idx UNIQUE (climb_location_id, user_id)
);
