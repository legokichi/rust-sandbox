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

pub enum OAuthProvider {
    Facebook(i64, String),
    Github(i64, String),
}

pub async fn create_user(
    pool: &sqlx::sqlite::SqlitePool,
    provider: OAuthProvider,
) -> Result<crate::model::user::User, anyhow::Error> {
    let mut tx = pool.begin().await?;
    match provider {
        OAuthProvider::Facebook(facebook_id, name) => {
            let user = sqlx::query_as!(
                crate::model::user::User,
                "SELECT users.* FROM facebook INNER JOIN users ON users.id = user_id WHERE facebook_id = ?1",
                facebook_id
            )
            .fetch_optional(&mut *tx)
            .await?;
            if let Some(user) = user {
                return Ok(user);
            }
            let user = sqlx::query_as!(
                crate::model::user::User,
                r#"INSERT INTO users DEFAULT VALUES RETURNING *"#
            )
            .fetch_one(&mut *tx)
            .await?;
            sqlx::query!(
                r#"INSERT INTO facebook ( user_id, facebook_id, name ) VALUES ( ?1, ?2, ?3 )"#,
                user.id,
                facebook_id,
                name
            )
            .execute(&mut *tx)
            .await?;
            tx.commit().await?;
            Ok(user)
        }
        OAuthProvider::Github(github_id, login) => {
            let user = sqlx::query_as!(
                crate::model::user::User,
                "SELECT users.* FROM github INNER JOIN users ON users.id = user_id WHERE github_id = ?1",
                github_id
            )
            .fetch_optional(&mut *tx)
            .await?;
            if let Some(user) = user {
                return Ok(user);
            }
            let user = sqlx::query_as!(
                crate::model::user::User,
                r#"INSERT INTO users DEFAULT VALUES RETURNING *"#
            )
            .fetch_one(&mut *tx)
            .await?;
            sqlx::query!(
                r#"INSERT INTO github ( user_id, github_id, login ) VALUES ( ?1, ?2, ?3 )"#,
                user.id,
                github_id,
                login
            )
            .execute(&mut *tx)
            .await?;
            tx.commit().await?;
            Ok(user)
        }
    }
}

// 多重ログイン
#[allow(dead_code)]
pub async fn update_user(
    pool: &sqlx::sqlite::SqlitePool,
    user_id: i64,
    provider: OAuthProvider,
) -> Result<(), anyhow::Error> {
    let mut tx = pool.begin().await?;
    match provider {
        OAuthProvider::Facebook(facebook_id, name) => {
            sqlx::query!(
                "INSERT INTO facebook ( user_id, facebook_id, name ) VALUES ( ?1, ?2, ?3 ) ON CONFLICT ( user_id ) DO UPDATE SET facebook_id = ?2, name = ?3, updated_at = strftime('%s', 'now')",
                user_id,
                facebook_id,
                name
            )
            .execute(&mut *tx)
            .await?;
            Ok(())
        }
        OAuthProvider::Github(github_id, login) => {
            sqlx::query!(
                "INSERT INTO github ( user_id, github_id, login ) VALUES ( ?1, ?2, ?3 ) ON CONFLICT ( user_id ) DO UPDATE SET github_id = ?2, login = ?3, updated_at = strftime('%s', 'now')",
                user_id,
                github_id,
                login
            )
            .execute(&mut *tx)
            .await?;
            Ok(())
        }
    }
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
