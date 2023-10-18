-- Your SQL goes here
ALTER TABLE ONLY climb_location_disciplines
    ADD CONSTRAINT "ClimbLocationDisciplines_climb_location_id_fkey" FOREIGN KEY (climb_location_id) REFERENCES climb_locations(id);

ALTER TABLE ONLY friends
    ADD CONSTRAINT "Friends_profile_one_fkey" FOREIGN KEY (profile_one) REFERENCES profiles(id);

ALTER TABLE ONLY friends
    ADD CONSTRAINT "Friends_profile_two_fkey" FOREIGN KEY (profile_two) REFERENCES profiles(id);

ALTER TABLE ONLY preferred_locations
    ADD CONSTRAINT "PreferredLocations_user_id_fkey" FOREIGN KEY (user_id) REFERENCES profiles(id);

ALTER TABLE ONLY session_participants
    ADD CONSTRAINT "SessionParticipants_session_id_fkey" FOREIGN KEY (session_id) REFERENCES sessions(id);

ALTER TABLE ONLY session_participants
    ADD CONSTRAINT "SessionParticipants_user_id_fkey" FOREIGN KEY (user_id) REFERENCES profiles(id);

ALTER TABLE ONLY sessions
    ADD CONSTRAINT "Sessions_author_id_fkey" FOREIGN KEY (author_id) REFERENCES profiles(id);

ALTER TABLE ONLY sessions
    ADD CONSTRAINT "Sessions_location_id_fkey" FOREIGN KEY (location_id) REFERENCES climb_locations(id);

ALTER TABLE ONLY user_disciplines
    ADD CONSTRAINT "UserDisciplines_user_id_fkey" FOREIGN KEY (user_id) REFERENCES profiles(id);
