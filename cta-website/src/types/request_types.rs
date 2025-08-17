use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct LoginPayload {
    pub username: String,
    pub pwd: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct LoginReturn {
    pub result: ResultReturn,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct ResultReturn {
    pub access_token: String,
    pub refresh_token: String,
    pub success: bool,
}
