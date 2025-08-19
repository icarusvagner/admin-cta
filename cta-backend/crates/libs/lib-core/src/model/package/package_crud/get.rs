use modql::filter::{FilterGroups, ListOptions};
use sea_query::{Condition, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow};

use crate::{
    ctx::Ctx,
    model::{
        base::compute_list_options,
        package::{Location, PackageBy, PackgeBmc, TblLocation},
        Error, ModelManager, Result,
    },
};

impl PackageBy for Location {}

impl PackgeBmc {
    pub async fn get_location_by_id<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: PackageBy,
    {
        // -- Build query
        let mut query = Query::select();
        query
            .from(TblLocation::Table)
            .columns([
                TblLocation::Id,
                TblLocation::Name,
                TblLocation::City,
                TblLocation::Province,
                TblLocation::Category,
                TblLocation::Description,
            ])
            .and_where(Expr::col(TblLocation::Id).eq(id));

        // -- Execute query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);

        let entity = mm
            .dbx()
            .fetch_optional(sqlx_query)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: "tbl_location",
                id,
            })?;

        Ok(entity)
    }

    pub async fn get_locations<E, F>(
        ctx: &Ctx,
        mm: &ModelManager,
        filter: Option<F>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<E>>
    where
        E: PackageBy,
        F: Into<FilterGroups>,
        E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    {
        let mut query = Query::select();
        query.from(TblLocation::Table).columns([
            TblLocation::Id,
            TblLocation::Name,
            TblLocation::City,
            TblLocation::City,
            TblLocation::Province,
            TblLocation::Description,
        ]);

        if let Some(filter) = filter {
            let filters: FilterGroups = filter.into();
            let cond: Condition = filters.try_into()?;
            query.cond_where(cond);
        }

        let list_options = compute_list_options(list_options)?;
        list_options.apply_to_sea_query(&mut query);

        // -- Execute query
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
        let entities = mm.dbx().fetch_all(sqlx_query).await?;

        Ok(entities)
    }
}
