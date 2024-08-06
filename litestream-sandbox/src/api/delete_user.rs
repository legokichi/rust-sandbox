#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Request {
    user_id: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Response {}

pub async fn delete_user(
    pool: &sqlx::sqlite::SqlitePool,
    Request { user_id }: Request,
) -> Result<Response, anyhow::Error> {
    crate::db::user::delete_user(pool, user_id).await?;
    Ok(Response {})
}
