use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use super::database_models::{ClimbLocation, Discipline, Profile, Session};
use super::location_models::LocationDetailsResponse;
use super::profile_models::ProfileSummaryResponse;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDetailsResponse {
    name: String,
    author_id: Uuid,
    is_public: bool,
    start_time: NaiveDateTime,
    end_time: Option<NaiveDateTime>,
    maximum_participants: Option<i32>,
    participants: Vec<ProfileSummaryResponse>,
    discipline: Discipline,
    location: LocationDetailsResponse,
}

impl From<(Session, Vec<Profile>, ClimbLocation)> for SessionDetailsResponse {
    fn from((session, participants, location): (Session, Vec<Profile>, ClimbLocation)) -> Self {
        SessionDetailsResponse {
            name: session.name,
            author_id: session.author_id,
            is_public: session.is_public,
            start_time: session.start_time,
            end_time: session.end_time,
            maximum_participants: session.maximum_participants,
            participants: participants.into_iter().map(|p| p.into()).collect(),
            discipline: session.discipline,
            location: location.into(),
        }
    }
}
