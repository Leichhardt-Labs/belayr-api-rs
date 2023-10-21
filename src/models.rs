use std::io::Write;

use crate::schema::sql_types::ClimbType;
use crate::schema::{
    climb_location_disciplines, climb_locations, friends, preferred_locations, profiles,
    session_participants, sessions, user_disciplines,
};

use chrono::NaiveDateTime;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::{prelude::*, serialize};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = ClimbType)]
pub enum Discipline {
    Bouldering,
    TopRope,
    Lead,
}

impl ToSql<ClimbType, Pg> for Discipline {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Discipline::Bouldering => out.write_all(b"bouldering")?,
            Discipline::TopRope => out.write_all(b"top_rope")?,
            Discipline::Lead => out.write_all(b"lead")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<ClimbType, Pg> for Discipline {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"bouldering" => Ok(Discipline::Bouldering),
            b"top_rope" => Ok(Discipline::TopRope),
            b"lead" => Ok(Discipline::Lead),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = climb_locations)]
pub struct ClimbLocation {
    pub id: Uuid,
    pub name: String,
    pub address_line_one: Option<String>,
    pub address_line_two: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = climb_location_disciplines)]
pub struct ClimbLocationDiscipline {
    pub id: Uuid,
    pub climb_location_id: Uuid,
    pub discipline: Discipline,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = friends)]
pub struct Friend {
    pub id: Uuid,
    pub profile_one: Uuid,
    pub profile_two: Uuid,
    pub is_accepted: Option<bool>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct Profile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = preferred_locations)]
pub struct PreferredLocation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub climb_location_id: Uuid,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = session_participants)]
pub struct SessionParticipant {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub location_id: Uuid,
    pub is_public: bool,
    pub discipline: Discipline,
    pub author_id: Uuid,
    pub maximum_participants: Option<i32>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = user_disciplines)]
pub struct UserDiscipline {
    pub id: Uuid,
    pub user_id: Uuid,
    pub discipline: Discipline,
}
