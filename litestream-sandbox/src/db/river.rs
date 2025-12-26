pub async fn list_rivers(
    pool: &sqlx::sqlite::SqlitePool,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<(Vec<crate::model::river::River>, u32), anyhow::Error> {
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);
    let rows = sqlx::query_as!(
        crate::model::river::River,
        r#"
        SELECT
            river_id,
            name
        FROM rivers
        ORDER BY river_id ASC
        LIMIT ?1
        OFFSET ?2
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let next = offset + rows.len() as u32;
    Ok((rows, next))
}

pub async fn list_river_waypoints(
    pool: &sqlx::sqlite::SqlitePool,
    river_id: i64,
    offset: Option<u32>,
    limit: Option<u32>,
) -> Result<(Vec<crate::model::river::RiverWaypoint>, u32), anyhow::Error> {
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);
    let rows = sqlx::query_as!(
        crate::model::river::RiverWaypoint,
        r#"
        SELECT
            river_id,
            river_waypoint_id,
            name,
            distance,
            latitude,
            longitude
        FROM river_waypoints
        WHERE river_id = ?1
        ORDER BY river_waypoint_id ASC
        LIMIT ?2
        OFFSET ?3"#,
        river_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let next = offset + rows.len() as u32;
    Ok((rows, next))
}

pub async fn create_river_waypoint(
    pool: &sqlx::sqlite::SqlitePool,
    river_id: i64,
    distance: f64,
    name: String,
    longitude: f64,
    latitude: f64,
) -> Result<i64, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO river_waypoints (river_id, distance, name, longitude, latitude)
        VALUES (?1, ?2, ?3, ?4, ?5)
        RETURNING river_waypoint_id;
        "#,
        river_id,
        distance,
        name,
        longitude,
        latitude
    )
    .fetch_one(pool)
    .await?;
    let river_waypoint_id = row.river_waypoint_id;
    Ok(river_waypoint_id)
}
