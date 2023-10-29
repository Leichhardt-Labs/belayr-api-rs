// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ClimbType"))]
    pub struct ClimbType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ClimbType;

    climb_location_disciplines (id) {
        id -> Uuid,
        climb_location_id -> Uuid,
        discipline -> ClimbType,
    }
}

diesel::table! {
    climb_locations (id) {
        id -> Uuid,
        name -> Varchar,
        address_line_one -> Nullable<Varchar>,
        address_line_two -> Nullable<Varchar>,
        city -> Varchar,
        state -> Varchar,
        postal_code -> Varchar,
        country -> Varchar,
    }
}

diesel::table! {
    friends (id) {
        id -> Uuid,
        profile_one -> Uuid,
        profile_two -> Uuid,
        is_accepted -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    preferred_locations (id) {
        id -> Uuid,
        user_id -> Uuid,
        climb_location_id -> Uuid,
    }
}

diesel::table! {
    profiles (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
    }
}

diesel::table! {
    session_participants (id) {
        id -> Uuid,
        session_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ClimbType;

    sessions (id) {
        id -> Uuid,
        name -> Varchar,
        location_id -> Uuid,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
        is_public -> Bool,
        discipline -> ClimbType,
        author_id -> Uuid,
        maximum_participants -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ClimbType;

    user_disciplines (id) {
        id -> Uuid,
        user_id -> Uuid,
        discipline -> ClimbType,
    }
}

diesel::joinable!(climb_location_disciplines -> climb_locations (climb_location_id));
diesel::joinable!(preferred_locations -> profiles (user_id));
diesel::joinable!(session_participants -> profiles (user_id));
diesel::joinable!(session_participants -> sessions (session_id));
diesel::joinable!(sessions -> climb_locations (location_id));
diesel::joinable!(sessions -> profiles (author_id));
diesel::joinable!(user_disciplines -> profiles (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    climb_location_disciplines,
    climb_locations,
    friends,
    preferred_locations,
    profiles,
    session_participants,
    sessions,
    user_disciplines,
);
