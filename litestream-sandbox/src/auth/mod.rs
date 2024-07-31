mod facebook;
mod github;
mod instagram;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub code: String,
    pub old_state: oauth2::CsrfToken,
    pub new_state: oauth2::CsrfToken,
    pub provider: OAuthProvider,
}

#[derive(Debug, Clone)]
pub enum OAuthProvider {
    Facebook,
    Github,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct BackendError(#[from] pub anyhow::Error);

#[derive(Debug, Clone)]
pub struct Backend {
    facebook: crate::auth::facebook::Backend,
    github: crate::auth::github::Backend,
}

impl Backend {
    pub fn new(
        db: sqlx::SqlitePool,
        facebook_client_id: oauth2::ClientId,
        facebook_client_secret: oauth2::ClientSecret,
        github_client_id: oauth2::ClientId,
        github_client_secret: oauth2::ClientSecret,
        redirect_url: oauth2::RedirectUrl,
    ) -> Self {
        Self {
            facebook: crate::auth::facebook::Backend::new(
                db.clone(),
                facebook_client_id,
                facebook_client_secret,
                redirect_url.clone(),
            ),
            github: crate::auth::github::Backend::new(
                db.clone(),
                github_client_id,
                github_client_secret,
                redirect_url,
            ),
        }
    }

    pub fn authorize_url(&self, provider: OAuthProvider) -> (oauth2::url::Url, oauth2::CsrfToken) {
        match provider {
            OAuthProvider::Facebook => self.facebook.authorize_url(),
            OAuthProvider::Github => self.github.authorize_url(),
        }
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
        match creds.provider {
            OAuthProvider::Facebook => self.facebook.authenticate(crate::auth::facebook::Credentials{
                code: creds.code,
                old_state: creds.old_state,
                new_state: creds.new_state,
            }).await.map_err(|o| o.0.into()),
            OAuthProvider::Github => self.github.authenticate(crate::auth::github::Credentials{
                code: creds.code,
                old_state: creds.old_state,
                new_state: creds.new_state,
            }).await.map_err(|o| o.0.into()),
        }
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self.facebook.get_user(user_id).await.map_err(|o| o.0)?;
        Ok(user)
    }
}
