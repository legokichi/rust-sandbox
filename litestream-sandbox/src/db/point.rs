pub async fn list_points(
    pool: &sqlx::sqlite::SqlitePool,
    query: &crate::model::PointsQuery,
) -> Result<(Vec<crate::model::Point>, u32), anyhow::Error> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);
    let rows = sqlx::query_as!(
        crate::model::Point,
        "SELECT * FROM points ORDER BY timestamp ASC LIMIT ?1 OFFSET ?2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let next = offset + rows.len() as u32;
    Ok((rows, next))
}

pub async fn create_point(
    pool: &sqlx::sqlite::SqlitePool,
    point: &crate::model::Point,
) -> Result<(), anyhow::Error> {
    sqlx::query!(
        "INSERT INTO points ( id, timestamp, text ) VALUES ( ?1, ?2, ?3 )",
        point.id,
        point.timestamp,
        point.text
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_point(
    pool: &sqlx::sqlite::SqlitePool,
    id: &str,
) -> Result<Option<crate::model::Point>, anyhow::Error> {
    let row = sqlx::query_as!(
        crate::model::Point,
        "SELECT * FROM points WHERE id = ?1",
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn update_point(
    pool: &sqlx::sqlite::SqlitePool,
    id: &str,
    point: &crate::model::UpdatePoint,
) -> Result<(), anyhow::Error> {
    let mut tx = pool.begin().await?;
    if let Some(text) = &point.text {
        sqlx::query!("UPDATE points SET text = ?1 WHERE id = ?2", text, id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn delete_point(pool: &sqlx::sqlite::SqlitePool, id: &str) -> Result<(), anyhow::Error> {
    sqlx::query!("DELETE FROM points WHERE id = ?1", id)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test(pool: sqlx::sqlite::SqlitePool) {
        dotenvy::dotenv().ok();
        env_logger::builder().is_test(true).try_init().ok();
        let point1 = crate::model::Point {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            text: "test".to_string(),
        };
        create_point(&pool, &point1).await.unwrap();
        {
            let (rows, next) = list_points(
                &pool,
                &crate::model::PointsQuery {
                    offset: Some(0),
                    limit: Some(1),
                },
            )
            .await
            .unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(next, 1);
        }
        {
            let row = get_point(&pool, &point1.id).await.unwrap();
            assert_eq!(row, Some(point1.clone()));
        }
        let point2 = crate::model::Point {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            text: "test".to_string(),
        };
        create_point(&pool, &point2).await.unwrap();
        {
            let (rows, next) = list_points(
                &pool,
                &crate::model::PointsQuery {
                    offset: Some(0),
                    limit: Some(1),
                },
            )
            .await
            .unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(next, 1);
            assert_eq!(rows.first().unwrap(), &point1);
            let (rows, next) = list_points(
                &pool,
                &crate::model::PointsQuery {
                    offset: Some(1),
                    limit: Some(1),
                },
            )
            .await
            .unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(rows.first().unwrap(), &point2);
            assert_eq!(next, 2);
        }
        delete_point(&pool, &point1.id).await.unwrap();
        let row = get_point(&pool, &point1.id).await.unwrap();
        assert_eq!(row, None);
    }
}
