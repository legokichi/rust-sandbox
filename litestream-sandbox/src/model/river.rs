#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct River {
    pub river_id: i64,
    pub name: String,
    // pub created_at: i64,
    // pub updated_at: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct RiverWaypoint {
    pub river_waypoint_id: i64,
    pub river_id: i64,
    pub name: String,
    // pub description: String,
    pub distance: f64,
    pub latitude: f64,
    pub longitude: f64,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub elevation: Option<f64>,
    // pub created_at: i64,
    // pub updated_at: i64,
}
