pub mod list_user;

pub async fn handler(
    st: &crate::web::State,
    user_id: i64,
    req: crate::model::api::Request,
) -> Result<crate::model::api::Response, anyhow::Error> {
    let mut conn = st.db.acquire().await?;
    crate::db::user::check_permission(&mut *conn, user_id, &req).await?;
    match req {
        crate::model::api::Request::ListUser(req) => {
            let res = crate::api::list_user::list_user(&st.db, req).await?;
            Ok(res.into())
        }
    }
}
