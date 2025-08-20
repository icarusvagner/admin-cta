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
    pub success: bool,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct WithIdReturn {
    pub result: ResultIdReturn,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct ResultIdReturn {
    pub success: bool,
    pub id: i64,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct LogoffReturn {
    pub result: LogoutResult,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct LogoutResult {
    pub logged_off: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct LogoffPayload {
    pub logoff: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct CreateLocationPayload {
    pub name: String,
    pub city: String,
    pub province: String,
    pub category: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct LocationReturn {
    pub result: LocationResult,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct LocationResult {
    pub success: bool,
    pub data: OneLocation,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct OneLocation {
    pub id: i64,
    pub name: String,
    pub city: String,
    pub province: String,
    pub category: String,
    pub description: String,
}
