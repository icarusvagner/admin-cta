use crate::{
    ctx::Ctx,
    model::{
        package::{
            InclusionForCreate, ItineraryDayForCreate, ItineraryDayLocationsForCreate,
            ItineraryDayOptionalActivityForCreate, LocationForCreate, OptionalActivityForCreate,
            PackageForCreate, PackageInclusionForCreate, PackageItineraryForCreate,
            PackagePricingForCreate, TblInclusion, TblItineraryDay, TblItineraryDayLocations,
            TblItineraryDayOptionalActivity, TblLocation, TblOptionalActivity, TblPackage,
            TblPackageInclusion, TblPackageItinerary, TblPackagePricing,
        },
        Error, ModelManager, Result,
    },
};
use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow};

pub trait PackageBy: for<'r> FromRow<'r, PgRow> + Unpin + Send {}

pub struct PackgeBmc;

impl PackgeBmc {
    pub async fn create_package(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: PackageForCreate,
    ) -> Result<i64> {
        let PackageForCreate {
            name,
            description,
            duration_days,
        } = data;

        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;
        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblPackage::Table)
            .columns([
                TblPackage::Name,
                TblPackage::Description,
                TblPackage::DurationDays,
            ])
            .values_panic([name.into(), description.into(), duration_days.into()])
            .returning_col(TblPackage::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_package" && constraint.contains("name") {
                        Some(Error::UniqueViolation {
                            table: "tbl_package".into(),
                            constraint: "name".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        mm.dbx().commit_txn().await?;

        Ok(id)
    }

    pub async fn create_location(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: LocationForCreate,
    ) -> Result<i64> {
        let LocationForCreate {
            name,
            city,
            province,
            category,
            description,
        } = data;

        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblLocation::Table)
            .columns([
                TblLocation::Name,
                TblLocation::City,
                TblLocation::Province,
                TblLocation::Category,
                TblLocation::Description,
            ])
            .values_panic([
                name.into(),
                city.into(),
                province.into(),
                category.into(),
                description.into(),
            ])
            .returning_col(TblLocation::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_location" && constraint.contains("name") {
                        Some(Error::UniqueViolation {
                            table: "tbl_location".into(),
                            constraint: "name".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        mm.dbx().commit_txn().await?;

        Ok(id)
    }

    pub async fn create_itinerary_day(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: ItineraryDayForCreate,
    ) -> Result<i64> {
        let ItineraryDayForCreate { name, description } = data;

        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblItineraryDay::Table)
            .columns([TblItineraryDay::Name, TblItineraryDay::Description])
            .values_panic([name.into(), description.into()])
            .returning_col(TblItineraryDay::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_itinerary_day" && constraint.contains("name") {
                        Some(Error::UniqueViolation {
                            table: "tbl_itinerary_day".into(),
                            constraint: "name".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        mm.dbx().commit_txn().await?;

        Ok(id)
    }

    pub async fn create_itinerary_day_location(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: ItineraryDayLocationsForCreate,
    ) -> Result<i64> {
        let ItineraryDayLocationsForCreate {
            itinerary_day_id,
            location_id,
            optional,
        } = data;

        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblItineraryDayLocations::Table)
            .columns([
                TblItineraryDayLocations::ItineraryDayId,
                TblItineraryDayLocations::LocationId,
                TblItineraryDayLocations::Optional,
            ])
            .values_panic([itinerary_day_id.into(), location_id.into(), optional.into()])
            .returning_col(TblItineraryDayLocations::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_itinerary_day_locations" && constraint.contains("name") {
                        Some(Error::UniqueViolation {
                            table: "tbl_itinerary_day_locations".into(),
                            constraint: "name".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        mm.dbx().commit_txn().await?;

        Ok(id)
    }

    pub async fn create_package_itinerary(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: PackageItineraryForCreate,
    ) -> Result<i64> {
        let PackageItineraryForCreate {
            package_id,
            itinerary_day_id,
        } = data;
        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblPackageItinerary::Table)
            .columns([
                TblPackageItinerary::ItineraryDayId,
                TblPackageItinerary::PackageId,
            ])
            .values_panic([package_id.into(), itinerary_day_id.into()])
            .returning_col(TblPackageItinerary::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_package_itinerary" && constraint.contains("package_id") {
                        Some(Error::UniqueViolation {
                            table: "tbl_package_itinerary".into(),
                            constraint: "package_id".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        mm.dbx().commit_txn().await?;

        Ok(id)
    }

    pub async fn create_package_pricing(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: PackagePricingForCreate,
    ) -> Result<i64> {
        let PackagePricingForCreate {
            package_id,
            pax_min,
            pax_max,
            price_per_pax,
        } = data;
        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblPackagePricing::Table)
            .columns([
                TblPackagePricing::PackageId,
                TblPackagePricing::PaxMin,
                TblPackagePricing::PaxMax,
                TblPackagePricing::PricePerPax,
            ])
            .values_panic([
                package_id.into(),
                pax_min.into(),
                pax_max.into(),
                price_per_pax.into(),
            ])
            .returning_col(TblPackagePricing::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_package_pricing" && constraint.contains("package_id") {
                        Some(Error::UniqueViolation {
                            table: "tbl_package_pricing".into(),
                            constraint: "package_id".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        Ok(id)
    }

    pub async fn create_optional_activity(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: OptionalActivityForCreate,
    ) -> Result<i64> {
        let OptionalActivityForCreate {
            name,
            surcharge_amount,
            unit,
            description,
        } = data;
        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblOptionalActivity::Table)
            .columns([
                TblOptionalActivity::Name,
                TblOptionalActivity::SurchargeAmount,
                TblOptionalActivity::Unit,
                TblOptionalActivity::Description,
            ])
            .values_panic([
                name.into(),
                surcharge_amount.into(),
                unit.into(),
                description.into(),
            ])
            .returning_col(TblOptionalActivity::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_optional_activity" && constraint.contains("name") {
                        Some(Error::UniqueViolation {
                            table: "tbl_optional_activity".into(),
                            constraint: "name".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        Ok(id)
    }

    pub async fn create_itinerary_day_optional_activity(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: ItineraryDayOptionalActivityForCreate,
    ) -> Result<i64> {
        let ItineraryDayOptionalActivityForCreate {
            itinerary_day_id,
            optional_activity_id,
        } = data;
        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblItineraryDayOptionalActivity::Table)
            .columns([
                TblItineraryDayOptionalActivity::ItineraryDayId,
                TblItineraryDayOptionalActivity::OptionalActivityId,
            ])
            .values_panic([itinerary_day_id.into(), optional_activity_id.into()])
            .returning_col(TblItineraryDayOptionalActivity::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_itinerary_day_optional_activity"
                        && constraint.contains("itinerary_day_id")
                        || constraint.contains("optional_activity_id")
                    {
                        Some(Error::UniqueViolation {
                            table: "tbl_itinerary_day_optional_activity".into(),
                            constraint: "itinerary_day_id or optional_activity_id".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        Ok(id)
    }

    pub async fn create_inclusion(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: InclusionForCreate,
    ) -> Result<i64> {
        let InclusionForCreate { name, description } = data;
        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let desc = if description.is_some() {
            description
        } else {
            Some("".to_string())
        };

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblInclusion::Table)
            .columns([TblInclusion::Name, TblInclusion::Description])
            .values_panic([name.into(), desc.into()])
            .returning_col(TblInclusion::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_inclusion" && constraint.contains("name") {
                        Some(Error::UniqueViolation {
                            table: "tbl_inclusion".into(),
                            constraint: "name".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        Ok(id)
    }

    pub async fn create_package_inclusion(
        _ctx: &Ctx,
        mm: &ModelManager,
        data: PackageInclusionForCreate,
    ) -> Result<i64> {
        let PackageInclusionForCreate {
            package_id,
            inclusion_id,
        } = data;
        let mm = mm.new_with_txn()?;
        mm.dbx().begin_txn().await?;

        let dbx = mm.dbx();
        let (sql, values) = Query::insert()
            .into_table(TblPackageInclusion::Table)
            .columns([
                TblPackageInclusion::PackageId,
                TblPackageInclusion::InclusionId,
            ])
            .values_panic([package_id.into(), inclusion_id.into()])
            .returning_col(TblPackageInclusion::Id)
            .build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, (i64,), _>(&sql, values);
        let (id,) = dbx.fetch_one(sqlx_query).await.map_err(|model_error| {
            Error::resolve_unique_violation(
                Error::Dbx(model_error),
                Some(|table: &str, constraint: &str| {
                    if table == "tbl_package_inclusion" && constraint.contains("package_id")
                        || constraint.contains("inclusion_id")
                    {
                        Some(Error::UniqueViolation {
                            table: "tbl_package_inclusion".into(),
                            constraint: "either package_id or inclusion_id".into(),
                        })
                    } else {
                        None
                    }
                }),
            )
        })?;

        Ok(id)
    }
}
