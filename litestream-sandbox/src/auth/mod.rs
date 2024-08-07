pub mod facebook;
pub mod github;
//pub mod instagram;

pub struct ClientToken {
    pub client_id: oauth2::ClientId,
    pub client_secret: oauth2::ClientSecret,
}

#[derive(Debug, Clone)]
pub struct Credentials {
    pub code: String,
    pub old_state: oauth2::CsrfToken,
    pub new_state: oauth2::CsrfToken,
    pub provider: OAuthProvider,
    pub user: Option<crate::model::user::User>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    Local,
    Github,
    Facebook,
    //Instagram,
}
impl std::str::FromStr for OAuthProvider {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(Self::Local),
            "github" => Ok(Self::Github),
            "facebook" => Ok(Self::Facebook),
            //"instagram" => Ok(Self::Instagram),
            _ => Err(anyhow::anyhow!("invalid OAuth provider: {}", s)),
        }
    }
}
impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Github => write!(f, "github"),
            Self::Facebook => write!(f, "facebook"),
            //Self::Instagram => write!(f, "instagram"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct BackendError(#[from] pub anyhow::Error);

#[derive(Debug, Clone)]
pub struct Backend {
    local: crate::auth::github::Backend,
    github: crate::auth::github::Backend,
    facebook: crate::auth::facebook::Backend,
    //instagram: crate::auth::instagram::Backend,
}

impl Backend {
    pub fn new(
        db: sqlx::SqlitePool,
        local: crate::auth::ClientToken,
        github: crate::auth::ClientToken,
        facebook: crate::auth::ClientToken,
        //instagram: crate::auth::instagram::ClientToken,
        redirect_url: oauth2::RedirectUrl,
    ) -> Self {
        Self {
            local: crate::auth::github::Backend::new(
                db.clone(),
                local,
                oauth2::RedirectUrl::new("http://127.0.0.1:8080/oauth/callback".to_string())
                    .unwrap(),
            ),
            github: crate::auth::github::Backend::new(db.clone(), github, redirect_url.clone()),
            facebook: crate::auth::facebook::Backend::new(
                db.clone(),
                facebook,
                redirect_url.clone(),
            ),
            //  instagram: crate::auth::instagram::Backend::new(
            //      db.clone(),
            //      instagram,
            //      redirect_url.clone(),
            //  ),
        }
    }

    pub fn authorize_url(&self, provider: OAuthProvider) -> (oauth2::url::Url, oauth2::CsrfToken) {
        match provider {
            OAuthProvider::Local => self.local.authorize_url(),
            OAuthProvider::Github => self.github.authorize_url(),
            OAuthProvider::Facebook => self.facebook.authorize_url(),
            //OAuthProvider::Instagram => self.instagram.authorize_url(),
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
            OAuthProvider::Local => self.local.authenticate(creds).await,
            OAuthProvider::Github => self.github.authenticate(creds).await,
            OAuthProvider::Facebook => self.facebook.authenticate(creds).await,
            //OAuthProvider::Instagram => self
            //    .instagram
            //    .authenticate(crate::auth::instagram::Credentials {
            //        code: creds.code,
            //        old_state: creds.old_state,
            //        new_state: creds.new_state,
            //    })
            //    .await
            //    .map_err(|o| o.0.into()),
        }
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self.github.get_user(user_id).await?;
        Ok(user)
    }
}
