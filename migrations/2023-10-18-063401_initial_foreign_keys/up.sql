-- Your SQL goes here
ALTER TABLE ONLY public.climb_location_disciplines
    ADD CONSTRAINT "ClimbLocationDisciplines_pkey" PRIMARY KEY (id);


--
-- Name: climb_locations ClimbLocations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.climb_locations
    ADD CONSTRAINT "ClimbLocations_pkey" PRIMARY KEY (id);


--
-- Name: friends Friends_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.friends
    ADD CONSTRAINT "Friends_pkey" PRIMARY KEY (id);


--
-- Name: preferred_locations PreferredLocations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.preferred_locations
    ADD CONSTRAINT "PreferredLocations_pkey" PRIMARY KEY (id);


--
-- Name: profiles Profiles_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.profiles
    ADD CONSTRAINT "Profiles_pkey" PRIMARY KEY (id);


--
-- Name: session_participants SessionParticipants_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.session_participants
    ADD CONSTRAINT "SessionParticipants_pkey" PRIMARY KEY (id);


--
-- Name: sessions Sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT "Sessions_pkey" PRIMARY KEY (id);


--
-- Name: user_disciplines UserDisciplines_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_disciplines
    ADD CONSTRAINT "UserDisciplines_pkey" PRIMARY KEY (id);


--
-- Name: Friends_profile_one_profile_two_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX "Friends_profile_one_profile_two_idx" ON public.friends USING btree (profile_one, profile_two);


--
-- Name: PreferredLocations_climb_location_id_user_id_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX "PreferredLocations_climb_location_id_user_id_idx" ON public.preferred_locations USING btree (climb_location_id, user_id);


--
-- Name: Profiles_email_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX "Profiles_email_key" ON public.profiles USING btree (email);


--
-- Name: Profiles_username_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX "Profiles_username_key" ON public.profiles USING btree (username);


--
-- Name: climb_location_disciplines ClimbLocationDisciplines_climb_location_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.climb_location_disciplines
    ADD CONSTRAINT "ClimbLocationDisciplines_climb_location_id_fkey" FOREIGN KEY (climb_location_id) REFERENCES public.climb_locations(id);


--
-- Name: friends Friends_profile_one_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.friends
    ADD CONSTRAINT "Friends_profile_one_fkey" FOREIGN KEY (profile_one) REFERENCES public.profiles(id);


--
-- Name: friends Friends_profile_two_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.friends
    ADD CONSTRAINT "Friends_profile_two_fkey" FOREIGN KEY (profile_two) REFERENCES public.profiles(id);


--
-- Name: preferred_locations PreferredLocations_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.preferred_locations
    ADD CONSTRAINT "PreferredLocations_user_id_fkey" FOREIGN KEY (user_id) REFERENCES public.profiles(id);


--
-- Name: session_participants SessionParticipants_session_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.session_participants
    ADD CONSTRAINT "SessionParticipants_session_id_fkey" FOREIGN KEY (session_id) REFERENCES public.sessions(id);


--
-- Name: session_participants SessionParticipants_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.session_participants
    ADD CONSTRAINT "SessionParticipants_user_id_fkey" FOREIGN KEY (user_id) REFERENCES public.profiles(id);


--
-- Name: sessions Sessions_author_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT "Sessions_author_id_fkey" FOREIGN KEY (author_id) REFERENCES public.profiles(id);


--
-- Name: sessions Sessions_location_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT "Sessions_location_id_fkey" FOREIGN KEY (location_id) REFERENCES public.climb_locations(id);


--
-- Name: user_disciplines UserDisciplines_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_disciplines
    ADD CONSTRAINT "UserDisciplines_user_id_fkey" FOREIGN KEY (user_id) REFERENCES public.profiles(id);


