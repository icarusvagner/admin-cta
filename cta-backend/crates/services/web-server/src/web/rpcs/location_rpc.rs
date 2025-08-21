use lib_core::model::{
    location::LocationBmc,
    package::{Location, LocationFilter, LocationForCreate, LocationForUpdate},
};

use lib_rpc_core::prelude::*;

pub fn rpc_router_builder() -> RouterBuilder {
    router_builder!(
        create_location,
        get_location,
        list_locations,
        update_location,
        delete_location,
        count_location
    )
}

generate_common_rpc_fns!(
    Bmc: LocationBmc,
    Entity: Location,
    ForCreate: LocationForCreate,
    ForUpdate: LocationForUpdate,
    Filter: LocationFilter,
    Suffix: location
);
