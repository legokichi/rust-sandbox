pub async fn list_rivers(
    pool: &sqlx::sqlite::SqlitePool,
    crate::model::api::list_rivers::Request { offset, limit }: crate::model::api::list_rivers::Request,
) -> Result<crate::model::api::list_rivers::Response, anyhow::Error> {
    let (rivers, next_offset) = crate::db::river::list_rivers(pool, offset, limit).await?;
    Ok(crate::model::api::list_rivers::Response {
        rivers,
        next: next_offset,
    })
}
