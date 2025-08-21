use std::borrow::Cow;

use derive_more::derive::From;
use lib_auth::pass;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::error::DatabaseError;

use super::store::dbx;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    #[from]
    TimeErrorComponentRange(#[serde_as(as = "DisplayFromStr")] time::error::ComponentRange),
    #[from]
    ChronoError(#[serde_as(as = "DisplayFromStr")] chrono::ParseError),
    InvalidBirthdateFormat,
    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    ListLimitOverMax {
        max: i64,
        actual: i64,
    },
    CountFail,
    AdminAlreadyExists {
        username: String,
    },
    UniqueViolation {
        table: String,
        constraint: String,
    },
    CantCreateModelManagerProvider(String),

    // --- For all data CRUD
    CantCreateData(String),
    CantRetrieveData(String),
    CantUpdateData(String),
    CantDeleteData(String),
    ConstraintErrorTable(String),

    #[from]
    Dbx(dbx::Error),

    #[from]
    Pwd(pass::Error),

    // CRUD Models error
    AccountIsRemoved {
        uname: String,
        email: String,
        id: i64,
    },
    AdminAlreadyRemoved {
        admin_id: i64,
    },
    InsertionFailed {
        entity: String,
        cause: String,
    },
    SelectionFailed {
        entity: String,
        cause: String,
    },
    DeletionFailed {
        entity: String,
        cause: String,
    },
    UpdatingFailed {
        entity: String,
        cause: String,
    },
    UsernameNotFound {
        entity: String,
        cause: String,
    },
    UsernameAlreadyExists {
        uname: String,
    },

    // General CID and MID
    CannotUpdateCidMid {
        entity: String,
        gen_id: i64,
    },

    // -- Externals
    #[from]
    ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),
    #[from]
    DatabaseError(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    #[from]
    SeaQueryError(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),
}

impl Error {
    /// This function will transform the error into a more precise variant if it is an SQLX or PGError Unique Violation.
    /// The resolver can contain a function (table_name: &str, constraint: &str) that may return a specific Error if desired.
    /// If the resolver is None, or if the resolver function returns None, it will default to Error::UniqueViolation {table, constraint}.
    pub fn resolve_unique_violation<F>(self, resolver: Option<F>) -> Self
    where
        F: FnOnce(&str, &str) -> Option<Self>,
    {
        match self
            .as_database_error()
            .map(|db_error| (db_error.code(), db_error.table(), db_error.constraint()))
        {
            // "23505" => postgresql "unique violation"
            Some((Some(Cow::Borrowed("23505")), Some(table), Some(constraint))) => resolver
                .and_then(|fun| fun(table, constraint))
                .unwrap_or_else(|| Error::UniqueViolation {
                    table: table.to_string(),
                    constraint: constraint.to_string(),
                }),
            _ => self,
        }
    }

    /// A convenient function to return the eventual database error (Postgres)
    /// if this Error is an SQLX Error that contains a database error.
    pub fn as_database_error(&self) -> Option<&(dyn DatabaseError + 'static)> {
        match self {
            Error::Dbx(dbx::Error::Sqlx(sqlx_error)) => sqlx_error.as_database_error(),
            _ => None,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::TimeErrorComponentRange(e) => {
                write!(f, "OffsetDateTime TimeErrorComponentRange - cause: {e}")
            }
            Error::ChronoError(e) => write!(f, "Chrono parsing error - cause: {e}"),
            Error::InvalidBirthdateFormat => write!(f, "Birth date is in invalid format"),
            Error::EntityNotFound { entity, id } => write!(
                f,
                "Database entity not found - entity: {entity:?} - id: {id}"
            ),
            Error::ListLimitOverMax { max, actual } => {
                write!(f, "List limit over max - max: {max} - actual: {actual}")
            }
            Error::CountFail => write!(f, "Count failed"),
            Error::AdminAlreadyExists { username } => write!(
                f,
                "Admin already exist in our records - username: {username:?}"
            ),
            Error::UniqueViolation { table, constraint } => write!(
                f,
                "Database unique violation occur - table: {table:?} - constraint: {constraint:?}"
            ),
            Error::CantCreateModelManagerProvider(cause) => {
                write!(f, "Cannot create model manager provider: {cause}")
            }

            Error::CantCreateData(cause) => write!(f, "Cannot create data: {cause}"),
            Error::CantRetrieveData(cause) => write!(f, "Cannot retrieve data: {cause}"),
            Error::CantUpdateData(cause) => write!(f, "Cannot update data: {cause}"),
            Error::CantDeleteData(cause) => write!(f, "Cannot delete data: {cause}"),
            Error::ConstraintErrorTable(cause) => write!(f, "Constraint table error: {cause}"),

            Error::Pwd(e) => write!(f, "Password error fail: {e}"),

            Error::AccountIsRemoved { uname, email, id } => {
                write!(f, "Account removed: {uname} email: {email} id: {id}")
            }
            Error::AdminAlreadyRemoved { admin_id } => {
                write!(f, "Admin is already removed: admin_id: {admin_id}")
            }
            Error::InsertionFailed { entity, cause } => {
                write!(f, "Insert failed: {entity} cause: {cause}")
            }
            Error::SelectionFailed { entity, cause } => {
                write!(f, "Selection failed: {entity} cause: {cause}")
            }
            Error::DeletionFailed { entity, cause } => {
                write!(f, "Deletion failed: {entity} cause: {cause}")
            }
            Error::UpdatingFailed { entity, cause } => {
                write!(f, "Updating failed: {entity} cause: {cause}")
            }
            Error::UsernameNotFound { entity, cause } => {
                write!(f, "Username not found: {entity} cause: {cause}")
            }
            Error::UsernameAlreadyExists { uname } => {
                write!(f, "Username is already in the record: {uname}")
            }

            Error::CannotUpdateCidMid { entity, gen_id } => write!(
                f,
                "Updating cid and mid failed - entity: {entity}, gen_id: {gen_id}"
            ),

            Error::ModqlIntoSea(e) => write!(f, "Modql Into Sea Error {e:?}"),
            Error::DatabaseError(e) => write!(f, "Sqlx error - cause: {e:?}"),
            Error::SeaQueryError(e) => write!(f, "SEA Query error - cause: {e:?}"),
            Error::Dbx(e) => write!(f, "Dbx error - cause: {e:?}"),
        }
    }
}

impl std::error::Error for Error {}
