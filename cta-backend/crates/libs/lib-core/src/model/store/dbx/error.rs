use derive_more::derive::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    TxnCantCommitNoOpenTxn,
    CannotBeginTxnWithTxnFalse,
    CannotCommitTxnWithTxnFalse,
    NoTxn,

    // -- Externals
    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::TxnCantCommitNoOpenTxn => write!(f, "TXN can't commit no open txn"),
            Error::CannotBeginTxnWithTxnFalse => write!(f, "Cannot begin txn with txn is false"),
            Error::CannotCommitTxnWithTxnFalse => write!(f, "Cannot commit txn with txn is false"),
            Error::NoTxn => write!(f, "No txn present"),
            Error::Sqlx(err) => write!(f, "SQLx error {err}"),
        }
    }
}

impl std::error::Error for Error {}
