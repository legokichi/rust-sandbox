pub async fn list_river_waypoints(
    pool: &sqlx::sqlite::SqlitePool,
    crate::model::api::list_river_waypoints::Request {
        offset,
        limit,
        river_id,
    }: crate::model::api::list_river_waypoints::Request,
) -> Result<crate::model::api::list_river_waypoints::Response, anyhow::Error> {
    let (river_waypoints, next_offset) =
        crate::db::river::list_river_waypoints(pool, river_id, offset, limit).await?;
    Ok(crate::model::api::list_river_waypoints::Response {
        river_waypoints,
        next: next_offset,
    })
}
