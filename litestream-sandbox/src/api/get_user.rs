#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Request {
    user_id: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Response {
    pub user: Option<crate::model::user::User>,
}

pub async fn get_user(
    pool: &sqlx::sqlite::SqlitePool,
    Request { user_id }: Request,
) -> Result<Response, anyhow::Error> {
    let res = crate::db::user::get_user(pool, user_id).await?;
    Ok(Response { user: res })
}
