pub async fn list_users(
    pool: &sqlx::sqlite::SqlitePool,
    crate::model::api::list_users::Request { offset, limit }: crate::model::api::list_users::Request,
) -> Result<crate::model::api::list_users::Response, anyhow::Error> {
    let (users, next_offset) = crate::db::user::list_users(pool, offset, limit).await?;
    Ok(crate::model::api::list_users::Response {
        users,
        next: next_offset,
    })
}
