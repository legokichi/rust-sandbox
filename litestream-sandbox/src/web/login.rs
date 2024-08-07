pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";
pub const PROVIDER_KEY: &str = "auth.provider";

#[derive(Debug, serde::Deserialize)]
pub struct LoginForm {
    provider: Option<crate::auth::OAuthProvider>,
}

/// POST /login
pub async fn login(
    auth_session: axum_login::AuthSession<crate::auth::Backend>,
    session: tower_sessions::Session,
    axum::Form(LoginForm { provider }): axum::Form<LoginForm>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    let Some(provider) = provider else {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "missing provider").into_response());
    };
    let (auth_url, csrf_state) = auth_session.backend.authorize_url(provider);
    session.insert(CSRF_STATE_KEY, csrf_state.secret()).await?;
    session.insert(PROVIDER_KEY, provider).await?;
    Ok(axum::response::Redirect::to(auth_url.as_str()).into_response())
}

// OAuth2 の認可コードを受け取るためのクエリパラメータ
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AuthzRequestQuery {
    pub code: String,
    pub state: oauth2::CsrfToken,
}

/// GET /oauth/callback
pub async fn callback(
    mut auth_session: axum_login::AuthSession<crate::auth::Backend>,
    session: tower_sessions::Session,
    axum::extract::Query(AuthzRequestQuery {
        code,
        state: new_state,
    }): axum::extract::Query<AuthzRequestQuery>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    use axum::response::IntoResponse;
    // セッションがない場合はエラー
    let Some(old_state) = session.get(CSRF_STATE_KEY).await? else {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "session expired").into_response());
    };
    let Some(provider) = session
        .get::<crate::auth::OAuthProvider>(PROVIDER_KEY)
        .await?
    else {
        return Ok((axum::http::StatusCode::BAD_REQUEST, "session expired").into_response());
    };
    let creds = crate::auth::Credentials {
        code,
        old_state,
        new_state,
        provider,
        // ログイン済みかどうか
        user: auth_session.user.clone(),
    };
    let Some(user) = auth_session.authenticate(creds.clone()).await? else {
        return Ok((
            axum::http::StatusCode::UNAUTHORIZED,
            "authentication failed",
        )
            .into_response());
    };
    auth_session.login(&user).await?;

    Ok(axum::response::Redirect::to("/").into_response())
}

/// POST /logout
pub async fn logout(
    mut auth_session: axum_login::AuthSession<crate::auth::Backend>,
    axum::Form(()): axum::Form<()>,
) -> Result<impl axum::response::IntoResponse, crate::web::Ise> {
    auth_session.logout().await?;
    Ok(axum::response::Redirect::to("/"))
}
