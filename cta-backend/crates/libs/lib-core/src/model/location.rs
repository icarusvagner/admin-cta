use crate::{
    ctx::Ctx,
    generate_common_bmc_fns,
    model::{
        base::{self, DbBmc},
        package::{Location, LocationFilter, LocationForCreate, LocationForUpdate},
        ModelManager, Result,
    },
};
use modql::filter::ListOptions;

pub struct LocationBmc;

impl DbBmc for LocationBmc {
    const TABLE: &'static str = "tbl_location";
}

generate_common_bmc_fns!(
    Bmc: LocationBmc,
    Entity: Location,
    ForCreate: LocationForCreate,
    ForUpdate: LocationForUpdate,
    Filter: LocationFilter,
);
