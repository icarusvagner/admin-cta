mod package_crud;

pub use package_crud::*;
use sea_query::Iden;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, sqlx::Type, derive_more::Display, Deserialize, Serialize)]
#[sqlx(type_name = "optional_flag")]
pub enum OptionalFlag {
    Yes,
    No,
}

#[derive(Iden)]
pub enum TblLocation {
    Table,
    Id,
    Name,
    City,
    Province,
    Category,
    Description,
}

#[derive(Iden)]
pub enum TblPackage {
    Table,
    Id,
    Name,
    Description,
    DurationDays,
}

#[derive(Iden)]
pub enum TblItineraryDay {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
pub enum TblItineraryDayLocations {
    Table,
    Id,
    ItineraryDayId,
    LocationId,
    Optional,
}

#[derive(Iden)]
pub enum TblPackageItinerary {
    Table,
    Id,
    PackageId,
    ItineraryDayId,
}

#[derive(Iden)]
pub enum TblPackagePricing {
    Table,
    Id,
    PackageId,
    PaxMin,
    PaxMax,
    PricePerPax,
}

#[derive(Iden)]
pub enum TblOptionalActivity {
    Table,
    Id,
    Name,
    SurchargeAmount,
    Unit, // e.g. 'per pax', 'per hour'
    Description,
}

#[derive(Iden)]
pub enum TblItineraryDayOptionalActivity {
    Table,
    Id,
    ItineraryDayId,
    OptionalActivityId,
}

#[derive(Iden)]
pub enum TblInclusion {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
pub enum TblPackageInclusion {
    Table,
    Id,
    PackageId,
    InclusionId,
}

#[derive(Iden)]
pub enum TblGuest {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    PhoneNumber,
    Country,
    CreatedAt,
}

#[derive(Deserialize)]
pub struct PackageForCreate {
    pub name: String,
    pub description: String,
    pub duration_days: i32,
}

#[derive(Deserialize)]
pub struct LocationForCreate {
    pub name: String,
    pub city: String,
    pub province: String,
    pub category: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ItineraryDayForCreate {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ItineraryDayLocationsForCreate {
    pub itinerary_day_id: i64,
    pub location_id: i64,
    pub optional: String,
}

#[derive(Deserialize)]
pub struct PackageItineraryForCreate {
    pub package_id: i64,
    pub itinerary_day_id: i64,
}

#[derive(Deserialize)]
pub struct PackagePricingForCreate {
    pub package_id: i64,
    pub pax_min: i32,
    pub pax_max: i32,
    pub price_per_pax: f32,
}

#[derive(Deserialize)]
pub struct OptionalActivityForCreate {
    pub name: String,
    pub surcharge_amount: f32,
    pub unit: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ItineraryDayOptionalActivityForCreate {
    pub itinerary_day_id: i64,
    pub optional_activity_id: i64,
}

#[derive(Deserialize)]
pub struct InclusionForCreate {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct PackageInclusionForCreate {
    pub package_id: i64,
    pub inclusion_id: i64,
}

pub struct GuestForCreate {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub country: String,
}
