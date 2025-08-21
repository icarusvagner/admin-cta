// region: Error

use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

// region: --- Error boilerplate

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    Key,
    Salt,
    Hash,
    PassValidate,
    SchemeNotFound(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::Key => write!(f, "Error with key"),
            Error::Salt => write!(f, "Error with salt"),
            Error::Hash => write!(f, "Error with hash"),
            Error::PassValidate => write!(f, "Error with password validation"),
            Error::SchemeNotFound(err) => write!(f, "Error with scheme not found - cause {err}"),
        }
    }
}

impl std::error::Error for Error {}

// endregion: --- Error boilerplate

// endregion: Error
