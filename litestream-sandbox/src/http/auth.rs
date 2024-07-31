pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";
pub const NEXT_URL_KEY: &str = "auth.next-url";

/// GET /
pub async fn index(
    auth_session: axum_login::AuthSession<crate::auth::Backend>,
) -> Result<impl axum::response::IntoResponse, crate::http::Ise> {
    use axum::response::IntoResponse;
    let user = auth_session.user.ok_or_else(|| anyhow::anyhow!("ise"))?;
    Ok(axum::response::Html(format!(
        r#"
<html>
<head>
    <title>OAuth2 Login</title>
</head>
<body>
    <form method="post" action="/login">
        <input type="submit" value="GitHub Login" />
        <input type="hidden" name="provider" value="github" />
    </form>
    <form method="post" action="/login">
        <input type="submit" value="Facebook Login" />
        <input type="hidden" name="provider" value="facebook" />
    </form>
    {}
</body>
</html>
    "#,
        &user.id
    ))
    .into_response())
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginQuery {
    next: Option<String>,
}

/// GET /login?next=https://...
pub async fn login_page(
    axum::extract::Query(LoginQuery { next }): axum::extract::Query<LoginQuery>,
) -> Result<impl axum::response::IntoResponse, crate::http::Ise> {
    let next_url = next
        .map(|next| format!(r#"<input type="hidden" name="next" value="{}" />"#, next))
        .unwrap_or_default();
    #[allow(clippy::useless_format)]
    Ok(axum::response::Html(format!(
        r#"
<html>
<head>
    <title>OAuth2 Login</title>
</head>
<body>
    <form method="post" action="/login">
        <input type="submit" value="GitHub Login" />
        <input type="hidden" name="provider" value="github" />
        {next_url}
    </form>
    <form method="post" action="/login">
        <input type="submit" value="Facebook Login" />
        <input type="hidden" name="provider" value="facebook" />
        {next_url}
    </form>
</body>
</html>
    "#
    )))
}
#[derive(Debug, serde::Deserialize)]
pub struct LoginForm {
    next: Option<String>,
    provider: Option<String>,
}

/// POST /login
pub async fn login(
    auth_session: axum_login::AuthSession<crate::auth::Backend>,
    session: tower_sessions::Session,
    axum::Form(LoginForm { next, provider }): axum::Form<LoginForm>,
) -> Result<impl axum::response::IntoResponse, crate::http::Ise> {
    use axum::response::IntoResponse;
    let Some(provider) = provider else {
        return Ok(axum::http::StatusCode::BAD_REQUEST.into_response());
    };
    let provider = match provider.as_str() {
        "github" => crate::auth::OAuthProvider::Github,
        "facebook" => crate::auth::OAuthProvider::Facebook,
        _ => return Ok(axum::http::StatusCode::BAD_REQUEST.into_response()),
    };
    let (auth_url, csrf_state) = auth_session.backend.authorize_url(provider);
    session.insert(CSRF_STATE_KEY, csrf_state.secret()).await?;
    session.insert(NEXT_URL_KEY, next).await?;
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
) -> Result<impl axum::response::IntoResponse, crate::http::Ise> {
    use axum::response::IntoResponse;
    let Some(old_state) = session.get(CSRF_STATE_KEY).await? else {
        return Ok(axum::http::StatusCode::BAD_REQUEST.into_response());
    };
    let creds = crate::auth::Credentials {
        code,
        old_state,
        new_state,
        provider: crate::auth::OAuthProvider::Github,
    };
    let Some(user) = auth_session.authenticate(creds).await? else {
        return Ok(axum::http::StatusCode::UNAUTHORIZED.into_response());
    };
    auth_session.login(&user).await?;

    if let Ok(Some(next)) = session.remove::<String>(NEXT_URL_KEY).await {
        return Ok(axum::response::Redirect::to(&next).into_response());
    }
    Ok(axum::response::Redirect::to("/").into_response())
}

/// GET /logout
pub async fn logout(
    mut auth_session: axum_login::AuthSession<crate::auth::Backend>,
) -> Result<impl axum::response::IntoResponse, crate::http::Ise> {
    auth_session.logout().await?;
    Ok(axum::response::Redirect::to("/login"))
}
