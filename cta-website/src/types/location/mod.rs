use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct LocationReturn {
    pub id: i64,
    pub jsonrpc: String,
    pub result: LocationResult,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct LocationResult {
    pub data: OneLocation,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct CountLocationReturn {
    pub id: i64,
    pub jsonrpc: String,
    pub result: CountLocationResult,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct CountLocationResult {
    pub data: i64,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct ListLocationReturn {
    pub id: i64,
    pub jsonrpc: String,
    pub result: ListLocationResult,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct ListLocationResult {
    pub data: Vec<OneLocation>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct OneLocation {
    pub id: i64,
    pub name: String,
    pub city: String,
    pub province: String,
    pub category: String,
    pub description: String,

    // -- Timestamps
    //    (creator and last modified user_id/time)
    pub cid: i64,
    pub ctime: String,
    pub mid: i64,
    pub mtime: String,
}

// ----
#[derive(Default, Serialize, Clone, PartialEq)]
pub struct LocationFilter {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub category: Option<String>,
}
