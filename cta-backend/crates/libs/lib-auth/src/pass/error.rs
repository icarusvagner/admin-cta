// region: --- Error

use super::scheme;
use derive_more::derive::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    PassWithSchemeFailedToParse,

    FailSpawnBlockForValidate,
    FailSpawnBlockForHash,

    // --- Modules
    #[from]
    Scheme(scheme::Error),

    // --- Uuid Error
    #[from]
    UUIDError(#[serde_as(as = "DisplayFromStr")] uuid::Error),
}

// endregion: --- Error boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::PassWithSchemeFailedToParse => write!(f, "Password with scheme failed to parse"),
            Error::FailSpawnBlockForValidate => write!(f, "Failed to spawn block for validation"),
            Error::FailSpawnBlockForHash => write!(f, "Failed to spawn block for hashing"),
            Error::Scheme(scheme) => write!(f, "Password scheme error {scheme}"),
            Error::UUIDError(uuid_err) => write!(f, "UUID error {uuid_err}"),
        }
    }
}

impl std::error::Error for Error {}

// endregion: --- Error
