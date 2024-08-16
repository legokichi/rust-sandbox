pub async fn list_access_logs(
    pool: &sqlx::sqlite::SqlitePool,
    crate::model::api::list_access_logs::Request {
        offset,
        limit,
        user_id,
    }: crate::model::api::list_access_logs::Request,
) -> Result<crate::model::api::list_access_logs::Response, anyhow::Error> {
    let (access_logs, next_offset) =
        crate::db::user::list_access_logs(pool, user_id, offset, limit).await?;
    Ok(crate::model::api::list_access_logs::Response {
        access_logs,
        next: next_offset,
    })
}
