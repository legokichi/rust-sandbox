pub async fn list_user(
    pool: &sqlx::sqlite::SqlitePool,
    crate::model::api::list_user::Request { offset, limit }: crate::model::api::list_user::Request,
) -> Result<crate::model::api::list_user::Response, anyhow::Error> {
    let (users, next_offset) = crate::db::user::list_users(pool, offset, limit).await?;
    Ok(crate::model::api::list_user::Response {
        users,
        next: next_offset,
    })
}
