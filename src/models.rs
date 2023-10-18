use super::schema::climb_locations;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
