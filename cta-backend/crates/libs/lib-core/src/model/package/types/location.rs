use crate::model::modql_utils::time_to_sea_value;
use lib_utils::time::Rfc3339;
use modql::{
    field::Fields,
    filter::{FilterNodes, OpValsInt64, OpValsString, OpValsValue},
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub city: String,
    pub province: String,
    pub category: String,
    pub description: String,

    // -- Timestamps
    //    (creator and last modified user_id/time)
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[derive(Deserialize, Fields)]
pub struct LocationForCreate {
    pub name: String,
    pub city: String,
    pub province: String,
    pub category: String,
    pub description: String,
}

#[derive(FilterNodes, Default, Deserialize)]
pub struct LocationFilter {
    pub id: Option<OpValsInt64>,
    pub name: Option<OpValsString>,

    pub cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

#[derive(Fields, Deserialize)]
pub struct LocationForUpdate {
    pub name: Option<String>,
}

// ---

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct ItineraryDay {
    pub id: i64,
    pub name: String,
    pub description: String,

    // -- Timestamps
    //    (creator and last modified user_id/time)
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct ItineraryDayLocations {
    pub id: i64,

    // -- Relations
    pub itinerary_day_id: i64,
    pub location_id: i64,
    pub optional: String,

    // -- Timestamps
    //    (creator and last modified user_id/time)
    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime,
}
