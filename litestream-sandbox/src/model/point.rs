#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct PointsQuery {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Point {
    // uuid
    pub id: i64,
    pub timestamp: i64,
    pub text: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct UpdatePoint {
    pub text: Option<String>,
}
