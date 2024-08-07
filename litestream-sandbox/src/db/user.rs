// SEE: https://github.com/launchbadge/sqlx/issues/1635#issuecomment-1027791249
#![allow(clippy::manual_async_fn)]

#[tracing::instrument(level = "trace", skip(conn))]
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
            github.id AS "github_id?",
            facebook.id AS "facebook_id?",
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuthProvider {
    Github(i64, String),
    Facebook(i64, String),
    //Instagram(i64, String),
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn create_user<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    provider: OAuthProvider,
) -> impl std::future::Future<Output = Result<crate::model::user::User, anyhow::Error>> + Send + 'a
{
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        let res = match provider {
            OAuthProvider::Github(github_id, login) => {
                create_user_by_github(&mut tx, github_id, login).await
            }
            OAuthProvider::Facebook(facebook_id, name) => {
                create_user_by_facebook(&mut tx, facebook_id, name).await
            } //OAuthProvider::Instagram(_instagram_id, _name) => {
              //    todo!()
              //}
        };
        tx.commit().await?;
        res
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
fn create_user_by_github<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    github_id: i64,
    login: String,
) -> impl std::future::Future<Output = Result<crate::model::user::User, anyhow::Error>> + Send + 'a
{
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        let user = sqlx::query_as!(
            crate::model::user::User,
            r#"
            SELECT
                users.id AS id,
                github.id AS github_id,
                facebook.id AS facebook_id,
                users.created_at AS created_at,
                users.updated_at AS updated_at
            FROM users
            LEFT OUTER JOIN github ON github.user_id = users.id
            LEFT OUTER JOIN facebook ON facebook.user_id = users.id
            WHERE github.id = ?1
            "#,
            github_id
        )
        .fetch_optional(&mut *tx)
        .await?;
        if let Some(user) = user {
            // 既に github アカウントで登録済みの場合はそのまま返す
            return Ok(user);
        }
        // 新規登録
        let user = sqlx::query!(
            r#"
            INSERT INTO users DEFAULT VALUES
            RETURNING id
            "#
        )
        .fetch_one(&mut *tx)
        .await?;
        // アカウント情報を登録
        sqlx::query!(
            r#"
            INSERT INTO github ( user_id, id, login )
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
}

#[tracing::instrument(level = "trace", skip(conn))]
fn create_user_by_facebook<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    facebook_id: i64,
    name: String,
) -> impl std::future::Future<Output = Result<crate::model::user::User, anyhow::Error>> + Send + 'a
{
    async move {
        use sqlx::Connection;
        let mut conn = conn.acquire().await?;
        let mut tx = conn.begin().await?;
        let user = sqlx::query_as!(
            crate::model::user::User,
            r#"
            SELECT
                users.id AS id,
                github.id AS github_id,
                facebook.id AS facebook_id,
                users.created_at AS created_at,
                users.updated_at AS updated_at
            FROM users
            LEFT OUTER JOIN github ON github.user_id = users.id
            LEFT OUTER JOIN facebook ON facebook.user_id = users.id
            WHERE facebook.id = ?1
            "#,
            facebook_id
        )
        .fetch_optional(&mut *tx)
        .await?;
        if let Some(user) = user {
            // 既に facebook アカウントで登録済みの場合はそのまま返す
            return Ok(user);
        }
        // 新規登録
        let user = sqlx::query!(
            r#"
            INSERT INTO users DEFAULT VALUES
            RETURNING id
            "#
        )
        .fetch_one(&mut *tx)
        .await?;
        // facebook アカウント情報を登録
        sqlx::query!(
            r#"
            INSERT INTO facebook ( user_id, id, name )
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
    }
}

// 多重ログイン
#[tracing::instrument(level = "trace", skip(conn))]
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
                    INSERT INTO github ( user_id, id, login )
                    VALUES ( ?1, ?2, ?3 )
                    ON CONFLICT ( user_id )
                    DO UPDATE SET id = ?2, login = ?3, updated_at = strftime('%s', 'now')
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
                    INSERT INTO facebook ( user_id, id, name )
                    VALUES ( ?1, ?2, ?3 )
                    ON CONFLICT ( user_id )
                    DO UPDATE SET id = ?2, name = ?3, updated_at = strftime('%s', 'now')
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
        tx.commit().await?;
        Ok(user)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
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
            github.id AS "github_id?",
            facebook.id AS "facebook_id?",
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

//#[tracing::instrument(level="trace", skip(conn))]
//pub fn delete_user<'a, 'c>(
//    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
//    id: i64,
//) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
//    async move {
//        let mut conn = conn.acquire().await?;
//        sqlx::query!(
//            r#"
//        DELETE
//        FROM users
//        WHERE id = ?1
//        "#,
//            id
//        )
//        .execute(&mut *conn)
//        .await?;
//        Ok(())
//    }
//}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn check_permission<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
    req: &'a crate::model::api::Request,
) -> impl std::future::Future<Output = Result<bool, anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            r#"
            SELECT
                roles.name AS role_name
            FROM users
            JOIN roles ON users.role_id = roles.id
            WHERE users.id = ?1
            "#,
            user_id
        )
        .fetch_one(&mut *conn)
        .await?;
        if row.role_name == "admin" {
            return Ok(true);
        }
        if row.role_name == "default" {
            let flag = match req {
                crate::model::api::Request::ListUser(..) => false,
            };
            return Ok(flag);
        }
        Ok(false)
    }
}

#[tracing::instrument(level = "trace", skip(conn))]
pub fn add_access_log<'a, 'c>(
    conn: impl sqlx::Acquire<'c, Database = sqlx::Sqlite> + Send + 'a,
    user_id: i64,
    req: &'a crate::model::api::Request,
) -> impl std::future::Future<Output = Result<(), anyhow::Error>> + Send + 'a {
    async move {
        let mut conn = conn.acquire().await?;
        let request = serde_json::to_string(req)?;
        sqlx::query!(
            r#"
            INSERT INTO access_logs ( user_id, request )
            VALUES ( ?1, ?2 )
            "#,
            user_id,
            request
        )
        .execute(&mut *conn)
        .await?;
        Ok(())
    }
}
