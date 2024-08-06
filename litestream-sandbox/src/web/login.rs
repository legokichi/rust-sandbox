pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";
pub const PROVIDER_KEY: &str = "auth.provider";

#[derive(Debug, serde::Deserialize)]
pub struct LoginForm {
    provider: Option<String>,
}

/// POST /login
pub async fn login(
    auth_session: axum_login::AuthSession<crate::auth::Backend>,
    session: tower_sessions::Session,
    axum::Form(LoginForm { provider }): axum::Form<LoginForm>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let Some(provider) = provider else {
        return Ok(axum::http::StatusCode::BAD_REQUEST.into_response());
    };
    let (auth_url, csrf_state) = auth_session.backend.authorize_url(provider.parse()?);
    session.insert(CSRF_STATE_KEY, csrf_state.secret()).await?;
    session.insert(PROVIDER_KEY, provider).await?;
    Ok(axum::response::Redirect::to(auth_url.as_str()).into_response())
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct AuthzRequest {
    pub code: String,
    pub state: oauth2::CsrfToken,
}

/// GET /oauth/callback
pub async fn callback(
    mut auth_session: axum_login::AuthSession<crate::auth::Backend>,
    session: tower_sessions::Session,
    axum::extract::Query(AuthzRequest {
        code,
        state: new_state,
    }): axum::extract::Query<AuthzRequest>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let Some(old_state) = session.get(CSRF_STATE_KEY).await? else {
        return Ok(axum::http::StatusCode::BAD_REQUEST.into_response());
    };
    let Some(provider) = session.get::<String>(PROVIDER_KEY).await? else {
        return Ok(axum::http::StatusCode::BAD_REQUEST.into_response());
    };
    let provider = provider.parse()?;
    let creds = crate::auth::Credentials {
        code,
        old_state,
        new_state,
        provider,
        user: auth_session.user.clone(),
    };
    let Some(user) = auth_session.authenticate(creds.clone()).await? else {
        return Ok(axum::http::StatusCode::UNAUTHORIZED.into_response());
    };
    auth_session.login(&user).await?;

    Ok(axum::response::Redirect::to("/").into_response())
}

/// GET /logout
pub async fn logout(
    mut auth_session: axum_login::AuthSession<crate::auth::Backend>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    auth_session.logout().await?;
    Ok(axum::response::Redirect::to("/"))
}
