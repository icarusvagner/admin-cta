use modql::SIden;
use sea_query::{Iden, IntoIden, TableRef};

mod crud_fns;
mod macro_utils;
mod utils;

pub use crud_fns::*;

const LIST_LIMIT_DEFAULT: i64 = 1000;
const LIST_LIMIT_MAX: i64 = 5000;

// region:    --- SeaQuery Idens

#[derive(Iden)]
pub enum CommonIden {
    Id,
    OwnerId,
}

#[derive(Iden)]
pub enum TimestampIden {
    Cid,
    Ctime,
    Mid,
    Mtime,
}

// endregion: --- SeaQuery Idens

pub trait DbBmc {
    const TABLE: &'static str;

    fn table_ref() -> TableRef {
        TableRef::Table(SIden(Self::TABLE).into_iden())
    }

    /// Specifies that the table for this Bmc has timestamps (cid, ctime, mid, mtime) columns.
    /// This will allow the code to update those as needed.
    ///
    /// default: true
    fn has_timestamps() -> bool {
        true
    }

    /// Specifies if the entity table managed by this BMC
    /// has an `owner_id` column that needs to be set on create (by default ctx.user_id).
    ///
    /// default: false
    fn has_owner_id() -> bool {
        false
    }
}
