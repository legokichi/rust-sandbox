#![allow(dead_code)]

const AUTH_URL: &str = "https://api.instagram.com/oauth/authorize";
const TOKEN_URL: &str = "https://api.instagram.com/oauth/access_token";

pub struct ClientToken {
    pub client_id: oauth2::ClientId,
    pub client_secret: oauth2::ClientSecret,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Credentials {
    pub code: String,
    pub old_state: oauth2::CsrfToken,
    pub new_state: oauth2::CsrfToken,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct BackendError(#[from] pub anyhow::Error);

#[derive(Debug, Clone)]
pub struct Backend {
    db: sqlx::SqlitePool,
    client: oauth2::basic::BasicClient,
}

impl Backend {
    pub fn new(
        db: sqlx::SqlitePool,
        client_token: ClientToken,
        redirect_url: oauth2::RedirectUrl,
    ) -> Self {
        let auth_url = oauth2::AuthUrl::new(AUTH_URL.to_string()).unwrap();
        let token_url = oauth2::TokenUrl::new(TOKEN_URL.to_string()).unwrap();

        let client = oauth2::basic::BasicClient::new(
            client_token.client_id,
            Some(client_token.client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);
        Self { db, client }
    }

    pub fn authorize_url(&self) -> (oauth2::url::Url, oauth2::CsrfToken) {
        self.client
            .authorize_url(oauth2::CsrfToken::new_random)
            .url()
    }
}

#[async_trait::async_trait]
impl axum_login::AuthnBackend for Backend {
    type User = crate::model::user::User;
    type Credentials = Credentials;
    type Error = BackendError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        use oauth2::TokenResponse;
        // Ensure the CSRF state has not been tampered with.
        if creds.old_state.secret() != creds.new_state.secret() {
            return Ok(None);
        };

        // Process authorization code, expecting a token response back.
        let token_res = self
            .client
            .exchange_code(oauth2::AuthorizationCode::new(creds.code))
            .request_async(|o| async {
                let res = oauth2::reqwest::async_http_client(o).await;
                log::debug!("{res:?}");
                res
            })
            .await
            .map_err(anyhow::Error::from)?;
        // Use access token to request user info.
        // https://developers.facebook.com/docs/instagram-basic-display-api/reference/me?locale=ja_JP
        #[derive(Debug, serde::Deserialize)]
        struct InstagramUserInfo {
            // legokichi
            name: String,
            // fb unique id
            id: i64,
        }

        let res = reqwest::Client::new()
            .get("https://graph.instagram.com/v20.0/me")
            .header(axum::http::header::USER_AGENT.as_str(), "axum-login")
            .header(
                axum::http::header::AUTHORIZATION.as_str(),
                format!("Bearer {}", token_res.access_token().secret()),
            )
            .send()
            .await;
        let user_info = res
            .map_err(anyhow::Error::from)?
            .text()
            .await
            .map_err(anyhow::Error::from)?;
        let user_info =
            serde_json::from_str::<InstagramUserInfo>(&user_info).map_err(anyhow::Error::from)?;

        // Persist user in our database so we can use `get_user`.
        let user = crate::db::user::create_user(
            &self.db,
            crate::db::user::OAuthProvider::Instagram(user_info.id, user_info.name),
        )
        .await
        .map_err(anyhow::Error::from)?;

        Ok(Some(user))
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = crate::db::user::get_user(&self.db, *user_id)
            .await
            .map_err(anyhow::Error::from)?;
        Ok(user)
    }
}
