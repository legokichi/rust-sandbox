#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    user: Option<crate::model::user::User>,
}

/// GET /
pub async fn index(
    auth_session: axum_login::AuthSession<crate::auth::Backend>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    let user = auth_session.user;
    Ok(IndexTemplate { user })
}
