mod error;

pub use self::error::{Error, Result};

// #[cfg_attr(feature = "with-rpc", derive(rpc_router::RpcResource))]
#[derive(Debug, Clone)]
pub struct Ctx {
    pub user_id: i64,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub uname: String,
}

impl AuthUser {
    pub fn root_ctx() -> Self {
        AuthUser {
            uname: String::new(),
        }
    }

    pub fn new(uname: String) -> Result<Self> {
        if uname.is_empty() {
            Err(Error::AuthUserCannotCreateNewRootUser)
        } else {
            Ok(Self { uname })
        }
    }
}

impl AuthUser {
    pub fn u_name(&self) -> String {
        self.uname.clone()
    }
}

impl Ctx {
    pub fn root_ctx() -> Self {
        Ctx { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRootCtx)
        } else {
            Ok(Self { user_id })
        }
    }
}

impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
