#[allow(dead_code)]
pub async fn list_users(
    pool: &sqlx::sqlite::SqlitePool,
    query: &crate::model::user::UserQuery,
) -> Result<(Vec<crate::model::user::User>, u32), anyhow::Error> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);
    let rows = sqlx::query_as!(
        crate::model::user::User,
        "SELECT * FROM users ORDER BY id, id ASC LIMIT ?1 OFFSET ?2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    let next = offset + rows.len() as u32;
    Ok((rows, next))
}

pub async fn create_user(
    pool: &sqlx::sqlite::SqlitePool,
    github_id: Option<i64>,
    facebook_id: Option<i64>,
) -> Result<crate::model::user::User, anyhow::Error> {
    let user = sqlx::query_as!(
        crate::model::user::User,
        "INSERT INTO users (github_id, facebook_id) VALUES (?1, ?2) RETURNING *",
        github_id,
        facebook_id
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn get_user(
    pool: &sqlx::sqlite::SqlitePool,
    id: i64,
) -> Result<Option<crate::model::user::User>, anyhow::Error> {
    let row = sqlx::query_as!(
        crate::model::user::User,
        "SELECT * FROM users WHERE id = ?1",
        id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

#[allow(dead_code)]
pub async fn delete_user(pool: &sqlx::sqlite::SqlitePool, id: &i64) -> Result<(), anyhow::Error> {
    sqlx::query!("DELETE FROM users WHERE id = ?1", id)
        .execute(pool)
        .await?;
    Ok(())
}
