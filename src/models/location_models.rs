use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::database_models::ClimbLocation;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationDetailsResponse {
    pub id: Uuid,
    pub name: String,
    pub address_line_one: String,
    pub address_line_two: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: String,
}

impl From<ClimbLocation> for LocationDetailsResponse {
    fn from(location: ClimbLocation) -> Self {
        LocationDetailsResponse {
            id: location.id,
            name: location.name,
            address_line_one: location.address_line_one.unwrap_or_default(),
            address_line_two: location.address_line_two.unwrap_or_default(),
            city: location.city,
            state: location.state,
            zip_code: location.postal_code,
            country: location.country,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationSummary {
    pub id: Uuid,
    pub name: String,
    pub address_line_one: String,
    pub address_line_two: String,
}

impl From<ClimbLocation> for LocationSummary {
    fn from(location: ClimbLocation) -> Self {
        LocationSummary {
            id: location.id,
            name: location.name,
            address_line_one: location.address_line_one.unwrap_or_default(),
            address_line_two: location.address_line_two.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLocationsRequest {
    #[validate(range(min = 1))]
    pub page: i64,
    #[validate(range(min = 1, max = 100))]
    pub page_size: i64,
}
