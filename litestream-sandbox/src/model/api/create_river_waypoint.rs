#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub river_id: i64,
    pub name: String,
    pub distance: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub river_waypoint_id: i64,
}
