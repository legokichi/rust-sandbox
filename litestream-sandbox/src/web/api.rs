#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum UnAuthRequest {}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub enum UnAuthResponse {}

/// POST /api
pub async fn api(
    auth_session: axum_login::AuthSession<crate::auth::Backend>,
    axum::extract::State(ref st): axum::extract::State<crate::web::State>,
    axum::extract::Json(json): axum::extract::Json<serde_json::Value>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let user = auth_session.user;
    match user {
        None => {
            let Ok(req) = serde_json::from_value::<UnAuthRequest>(json) else {
                return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid request").into_response());
            };
            match req {}
        }
        Some(user) => {
            let Ok(req) = serde_json::from_value::<crate::model::api::Request>(json) else {
                return Ok((axum::http::StatusCode::BAD_REQUEST, "invalid request").into_response());
            };
            let res = crate::api::handler(st, user.id, req).await?;
            let json = serde_json::to_value(res)?;
            Ok(axum::response::Json::from(json).into_response())
        }
    }
}
