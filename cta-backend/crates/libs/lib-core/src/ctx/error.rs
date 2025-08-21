use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
    CtxCannotNewRootCtx,
    AuthUserCannotCreateNewRootUser,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::CtxCannotNewRootCtx => write!(f, "Context can't create new root context"),
            Error::AuthUserCannotCreateNewRootUser => {
                write!(f, "Auth user cannot create new root user")
            }
        }
    }
}

impl std::error::Error for Error {}
