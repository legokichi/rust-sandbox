#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct PointsQuery {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Point {
    // uuid
    pub id: String,
    // iso8601 utc
    pub timestamp: String,
    pub text: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct UpdatePoint {
    pub text: Option<String>,
}