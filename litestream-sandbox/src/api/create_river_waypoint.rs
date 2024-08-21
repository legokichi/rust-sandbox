pub async fn create_river_waypoint(
    pool: &sqlx::sqlite::SqlitePool,
    crate::model::api::create_river_waypoint::Request {
        river_id,
        distance,
        name,
        longitude,
        latitude,
    }: crate::model::api::create_river_waypoint::Request,
) -> Result<crate::model::api::create_river_waypoint::Response, anyhow::Error> {
    let river_waypoint_id = crate::db::river::create_river_waypoint(
        pool, river_id, distance, name, longitude, latitude,
    )
    .await?;
    Ok(crate::model::api::create_river_waypoint::Response { river_waypoint_id })
}
