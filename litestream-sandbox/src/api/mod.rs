pub mod create_river_waypoint;
pub mod list_access_logs;
pub mod list_river_waypoints;
pub mod list_rivers;
pub mod list_users;

pub async fn handler(
    st: &crate::web::State,
    user_id: i64,
    req: crate::model::api::Request,
) -> Result<crate::model::api::Response, anyhow::Error> {
    let mut conn = st.db.acquire().await?;
    if !crate::db::user::check_permission(&mut *conn, user_id, &req).await? {
        return Ok(crate::model::api::ErrorKind::PermissionDenied.into());
    }
    crate::db::user::add_access_log(&mut *conn, user_id, &req).await?;
    match req {
        crate::model::api::Request::ListUsers(req) => {
            let res = crate::api::list_users::list_users(&st.db, req).await?;
            Ok(res.into())
        }
        crate::model::api::Request::ListAccessLogs(req) => {
            let res = crate::api::list_access_logs::list_access_logs(&st.db, req).await?;
            Ok(res.into())
        }
        crate::model::api::Request::ListRivers(req) => {
            let res = crate::api::list_rivers::list_rivers(&st.db, req).await?;
            Ok(res.into())
        }
        crate::model::api::Request::ListRiverWaypoints(req) => {
            let res = crate::api::list_river_waypoints::list_river_waypoints(&st.db, req).await?;
            Ok(res.into())
        }
        crate::model::api::Request::CreateRiverWaypoint(req) => {
            let res = crate::api::create_river_waypoint::create_river_waypoint(&st.db, req).await?;
            Ok(res.into())
        }
    }
}
