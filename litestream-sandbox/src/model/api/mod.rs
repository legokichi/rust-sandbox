pub mod list_access_logs;
pub mod list_river_waypoints;
pub mod list_rivers;
pub mod list_users;
// pub mod update_river_waypoint;
pub mod create_river_waypoint;

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    derive_more::TryInto,
    derive_more::From,
)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Request {
    ListUsers(crate::model::api::list_users::Request),
    ListAccessLogs(crate::model::api::list_access_logs::Request),
    ListRivers(crate::model::api::list_rivers::Request),
    ListRiverWaypoints(crate::model::api::list_river_waypoints::Request),
    // UpdateRiverWaypoint(crate::model::api::update_river_waypoint::Request),
    CreateRiverWaypoint(crate::model::api::create_river_waypoint::Request),
}

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    derive_more::TryInto,
    derive_more::From,
)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Response {
    ListUser(crate::model::api::list_users::Response),
    ListAccessLogs(crate::model::api::list_access_logs::Response),
    ListRivers(crate::model::api::list_rivers::Response),
    ListRiverWaypoints(crate::model::api::list_river_waypoints::Response),
    // UpdateRiverWaypoint(crate::model::api::update_river_waypoint::Response),
    CreateRiverWaypoint(crate::model::api::create_river_waypoint::Response),
    Error(ErrorKind),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "errorType")]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    PermissionDenied,
    InvalidRequest,
}

impl Request {
    #[allow(dead_code)]
    pub fn to_request_type_string(&self) -> Result<String, anyhow::Error> {
        use anyhow::Context;
        let json = serde_json::to_value(self)?;
        let json = json.pointer(".type").context("type field not found")?;
        let txt = json.as_str().context("type field is not a string")?;
        Ok(txt.to_string())
    }
}

impl Response {
    #[allow(dead_code)]
    pub fn to_response_type_string(&self) -> Result<String, anyhow::Error> {
        use anyhow::Context;
        let json = serde_json::to_value(self)?;
        let json = json.pointer(".type").context("type field not found")?;
        let txt = json.as_str().context("type field is not a string")?;
        Ok(txt.to_string())
    }
}
