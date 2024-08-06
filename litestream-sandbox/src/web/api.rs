#[derive(Debug, serde::Deserialize)]
pub struct Param {}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum UnAuthRequest {}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum UnAuthResponse {}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum Request {
    ListUser(crate::api::list_user::Request),
    UpdateUser(crate::api::update_user::Request),
    GetUser(crate::api::get_user::Request),
    DeleteUser(crate::api::delete_user::Request),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum Response {
    ListUser(crate::api::list_user::Response),
    UpdateUser(crate::api::update_user::Response),
    GetUser(crate::api::get_user::Response),
    DeleteUser(crate::api::delete_user::Response),
}

/// POST /api
pub async fn api(
    auth_session: axum_login::AuthSession<crate::auth::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    axum::extract::Query(Param {}): axum::extract::Query<Param>,
    axum::extract::Json(json): axum::extract::Json<serde_json::Value>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    let user = auth_session.user;
    match user {
        None => {
            let req = serde_json::from_value::<UnAuthRequest>(json)?;
            match req {
                _ => Ok(axum::response::Json::from(serde_json::json!({}))),
            }
        }
        Some(_user) => {
            let req = serde_json::from_value::<Request>(json)?;
            match req {
                Request::UpdateUser(req) => {
                    let res = crate::api::update_user::update_user(&st.db, req).await?;
                    Ok(axum::response::Json::from(serde_json::to_value(res)?))
                }
                Request::DeleteUser(req) => {
                    let res = crate::api::delete_user::delete_user(&st.db, req).await?;
                    Ok(axum::response::Json::from(serde_json::to_value(res)?))
                }
                Request::GetUser(req) => {
                    let res = crate::api::get_user::get_user(&st.db, req).await?;
                    Ok(axum::response::Json::from(serde_json::to_value(res)?))
                }
                Request::ListUser(req) => {
                    let res = crate::api::list_user::list_user(&st.db, req).await?;
                    Ok(axum::response::Json::from(serde_json::to_value(res)?))
                }
            }
        }
    }
}
