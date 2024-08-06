#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Request {
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Response {
    pub users: Vec<crate::model::user::User>,
    pub next: u32,
}

pub async fn list_user(
    pool: &sqlx::sqlite::SqlitePool,
    Request { offset, limit }: Request,
) -> Result<Response, anyhow::Error> {
    let (users, next_offset) = crate::db::user::list_users(pool, offset, limit).await?;
    Ok(Response {
        users,
        next: next_offset,
    })
}
