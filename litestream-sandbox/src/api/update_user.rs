#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Request {
    pub user_id: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Response {}

pub async fn update_user(
    pool: &sqlx::sqlite::SqlitePool,
    Request { user_id }: Request,
) -> Result<Response, anyhow::Error> {
    crate::db::user::update_user(pool, user_id, None).await?;
    Ok(Response {})
}
