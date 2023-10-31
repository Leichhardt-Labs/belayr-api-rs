use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use super::database_models::{Discipline, Profile};
use super::generic_models::PagedResponse;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDetailsResponse {
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub disciplines: Vec<Discipline>,
}

impl From<(Profile, Vec<Discipline>)> for ProfileDetailsResponse {
    fn from((profile, disciplines): (Profile, Vec<Discipline>)) -> Self {
        ProfileDetailsResponse {
            id: profile.id,
            username: profile.username,
            first_name: profile.first_name,
            last_name: profile.last_name,
            disciplines,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProfileSessionsResponse(PagedResponse<ProfileSession>);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSession {
    pub id: Uuid,
    pub author_id: Uuid,
    pub name: String,
    pub location_id: Uuid,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_public: bool,
    pub discipline: Discipline,
    pub maximum_participants: Option<i32>,
    pub current_participants: i32,
}
