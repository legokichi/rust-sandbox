// SEE: https://github.com/launchbadge/sqlx/issues/1635#issuecomment-1027791249
#![allow(clippy::manual_async_fn)]

use crate::api::get_user;

pub fn list_users<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    offset: Option<u32>,
    limit: Option<u32>,
) -> impl std::future::Future<Output = Result<(Vec<crate::model::user::User>, u32), anyhow::Error>>
       + Send
       + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        let rows = sqlx::query_as!(
            crate::model::user::User,
            r#"
        SELECT
            users.id AS id,
            github.github_id AS github_id,
            facebook.facebook_id AS facebook_id,
            users.created_at AS created_at,
            users.updated_at AS updated_at
        FROM users
        LEFT OUTER JOIN github ON users.id = github.user_id
        LEFT OUTER JOIN facebook ON users.id = facebook.user_id
        ORDER BY id, id ASC
        LIMIT ?1 OFFSET ?2
        "#,
            limit,
            offset
        )
        .fetch_all(&mut *conn)
        .await?;
        let next_offset = offset + rows.len() as u32;
        Ok((rows, next_offset))
    }
}

pub enum OAuthProvider {
    Github(i64, String),
    Facebook(i64, String),
    //Instagram(i64, String),
}

pub fn create_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    provider: OAuthProvider,
) -> impl std::future::Future<Output = Result<crate::model::user::User, anyhow::Error>> + Send + 'a
{
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        match provider {
            OAuthProvider::Github(github_id, login) => {
                let user = sqlx::query_as!(
                    crate::model::user::User,
                    r#"
                SELECT
                    users.id AS id,
                    github.github_id AS github_id,
                    facebook.facebook_id AS facebook_id,
                    users.created_at AS created_at,
                    users.updated_at AS updated_at
                FROM users
                LEFT OUTER JOIN github ON users.id = github.user_id
                LEFT OUTER JOIN facebook ON users.id = facebook.user_id
                WHERE github.id = ?1
                "#,
                    github_id
                )
                .fetch_optional(&mut *tx)
                .await?;
                if let Some(user) = user {
                    return Ok(user);
                }
                let user = sqlx::query!(
                    r#"
                INSERT INTO users DEFAULT VALUES
                RETURNING id
                "#
                )
                .fetch_one(&mut *tx)
                .await?;
                sqlx::query!(
                    r#"
                INSERT INTO github ( user_id, github_id, login )
                VALUES ( ?1, ?2, ?3 )
                "#,
                    user.id,
                    github_id,
                    login
                )
                .execute(&mut *tx)
                .await?;
                let user = get_user(&mut tx, user.id).await?.unwrap();
                tx.commit().await?;
                Ok(user)
            }
            OAuthProvider::Facebook(facebook_id, name) => {
                let user = sqlx::query_as!(
                    crate::model::user::User,
                    r#"
                SELECT
                    users.id AS id,
                    github.github_id AS github_id,
                    facebook.facebook_id AS facebook_id,
                    users.created_at AS created_at,
                    users.updated_at AS updated_at
                FROM users
                LEFT OUTER JOIN github ON users.id = github.user_id
                LEFT OUTER JOIN facebook ON users.id = facebook.user_id
                WHERE facebook.id = ?1
                "#,
                    facebook_id
                )
                .fetch_optional(&mut *tx)
                .await?;
                if let Some(user) = user {
                    return Ok(user);
                }
                let user = sqlx::query!(
                    r#"
                INSERT INTO users DEFAULT VALUES
                RETURNING id
                "#
                )
                .fetch_one(&mut *tx)
                .await?;
                sqlx::query!(
                    r#"
                INSERT INTO facebook ( user_id, facebook_id, name )
                VALUES ( ?1, ?2, ?3 )
                "#,
                    user.id,
                    facebook_id,
                    name
                )
                .execute(&mut *tx)
                .await?;
                let user = get_user(&mut tx, user.id).await?.unwrap();
                tx.commit().await?;
                Ok(user)
            } //OAuthProvider::Instagram(_instagram_id, _name) => {
              //    todo!()
              //}
        }
    }
}

// 多重ログイン
pub fn update_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
    provider: Option<OAuthProvider>,
) -> impl std::future::Future<Output = Result<crate::model::user::User, anyhow::Error>> + Send + 'a
{
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        if let Some(provider) = provider {
            match provider {
                OAuthProvider::Github(github_id, login) => {
                    sqlx::query!(
                        r#"
                    INSERT INTO github ( user_id, github_id, login )
                    VALUES ( ?1, ?2, ?3 )
                    ON CONFLICT ( user_id )
                    DO UPDATE SET github_id = ?2, login = ?3, updated_at = strftime('%s', 'now')
                    "#,
                        user_id,
                        github_id,
                        login
                    )
                    .execute(&mut *tx)
                    .await?;
                }
                OAuthProvider::Facebook(facebook_id, name) => {
                    sqlx::query!(
                        r#"
                    INSERT INTO facebook ( user_id, facebook_id, name )
                    VALUES ( ?1, ?2, ?3 )
                    ON CONFLICT ( user_id )
                    DO UPDATE SET facebook_id = ?2, name = ?3, updated_at = strftime('%s', 'now')
                    "#,
                        user_id,
                        facebook_id,
                        name
                    )
                    .execute(&mut *tx)
                    .await?;
                } //OAuthProvider::Instagram(instagram_id, name) => {
                  //    sqlx::query!(
                  //        r#"
                  //    INSERT INTO instagram ( user_id, instagram_id, name )
                  //    VALUES ( ?1, ?2, ?3 )
                  //    ON CONFLICT ( user_id )
                  //    DO UPDATE SET instagram_id = ?2, name = ?3, updated_at = strftime('%s', 'now')
                  //    "#,
                  //        user_id,
                  //        instagram_id,
                  //        name
                  //    )
                  //    .execute(&mut *tx)
                  //    .await?;
                  //}
            }
        }
        let user = get_user(&mut tx, user_id).await?.unwrap();
        Ok(user)
    }
}

pub fn get_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    id: i64,
) -> impl std::future::Future<Output = Result<Option<crate::model::user::User>, anyhow::Error>> + Send + 'a
{
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query_as!(
            crate::model::user::User,
            r#"
        SELECT 
            users.id AS id,
            github.github_id AS github_id,
            facebook.facebook_id AS facebook_id,
            users.created_at AS created_at,
            users.updated_at AS updated_at
        FROM users 
        LEFT OUTER JOIN github ON users.id = github.user_id
        LEFT OUTER JOIN facebook ON users.id = facebook.user_id
        WHERE users.id = ?1
        "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await?;
        Ok(row)
    }
}

pub fn delete_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    id: i64,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        sqlx::query!(
            r#"
        DELETE
        FROM users
        WHERE id = ?1
        "#,
            id
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}
