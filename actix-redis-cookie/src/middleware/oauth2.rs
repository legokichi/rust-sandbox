use super::*;

use actix_web::client::ClientRequest;

pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
    pub origin: ::url::Url,
    pub authorize_endpoint: String,
    pub access_token_endpoint: String,
    pub login_path: String,
    pub callback_path: String,
}
/// * https://developer.github.com/apps/building-oauth-apps/authorizing-oauth-apps/
/// * アクセストークンに寿命はない - https://developer.github.com/v3/oauth_authorizations/#check-an-authorization
/// * アクセストークンに有効期限を設けるべきかどうか - https://qiita.com/r7kamura/items/3e03471e02ea9ab5902a
/// * https://docs.kii.com/ja/guides/cloudsdk/rest/managing-users/access-token/refresh-token/
/// * https://auth0.com/blog/jp-refresh-tokens-what-are-they-and-when-to-use-them/
pub struct OAuth2Middleware {
    connector: ::actix::Addr<::actix_web::client::ClientConnector>,
    client_id: String,
    client_secret: String,
    /// req.session().get(&self.session_key)
    session_key: String,
    /// ex. "https://localhost:8080"
    origin: ::url::Url,
    /// ex. "/auth/auth/github"
    login_path: String,
    /// ex. "/auth/cb/github"
    callback_path: String,
    /// ex. "user:email""
    scope: String,
    /// ex. "https://github.com/login/oauth/authorize"
    authorize_endpoint: String,
    /// ex. "https://github.com/login/oauth/access_token"
    access_token_endpoint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken { 
    pub access_token: String,
    pub token_type: String,
}

impl OAuth2Middleware {
    pub fn new(connector: ::actix::Addr<::actix_web::client::ClientConnector>, config: OAuth2Config) -> Self {
        OAuth2Middleware {
            connector,
            client_id: config.client_id,
            client_secret: config.client_secret,
            session_key: "user_id".into(),
            origin: config.origin,
            login_path: config.login_path,
            callback_path: config.callback_path,
            scope: config.scope,
            authorize_endpoint: config.authorize_endpoint,
            access_token_endpoint: config.access_token_endpoint,
        }
    }
    fn handle_login<S>(&self, req: &HttpRequest<S>) -> Result<Started, ::actix_web::Error> {
        let mut redirect_url = self.origin.clone();
        redirect_url.set_path(self.login_path.as_str());
        let mut authorize_url = ::url::Url::parse(self.authorize_endpoint.as_str()).map_err(ErrorInternalServerError)?;
        authorize_url.query_pairs_mut()
            .append_pair("client_id", self.client_id.as_str())
            .append_pair("scope", "user")
            .append_pair("state", &::uuid::Uuid::new_v4().to_string())
            .append_pair("redirect_uri", redirect_url.as_str());
        let resp = HttpResponse::Found().header("location", authorize_url.as_str()).finish();
        return Ok(Started::Response(resp));
    }
    fn handle_callback<S>(&self, req: &HttpRequest<S>) -> Result<Started, ::actix_web::Error> {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(untagged)]
        pub enum CallbackResult {
            Ok{ code: String, state: String },
            Err{ error: String, error_description: String, error_uri: String, state: String }
        }
        let query = <Query<CallbackResult> as FromRequest<S>>::extract(req)?;
        let ( code, state ) = {
            match query.into_inner() {
                err @ CallbackResult::Err{ .. } => {
                    let resp = HttpResponse::InternalServerError().json(err);
                    return Ok(Started::Response(resp));
                },
                CallbackResult::Ok{ code, state } => ( code, state ),
            }
        };
        let session = req.session();
        let session_key = self.session_key.clone();
        let conn = self.connector.clone();
        let fut = mdo!{
            token =<< self.request_access_token(code, state);
            user_data =<< github_api::get_user_data(conn, &token);
            () =<< future::result(session.set(&session_key, user_data.id)).from_err();
            ret ret(Some(HttpResponse::SeeOther().header("location", "/content").finish()))
        };
        return Ok(Started::Future(Box::new(fut)));
    }
    fn request_access_token(&self, code: String, state: String) -> impl Future<Item=AccessToken, Error=::actix_web::Error> {
        mdo!{
            let encoded = ::url::form_urlencoded::Serializer::new(String::new())
                .append_pair("client_id", self.client_id.as_str())
                .append_pair("client_secret", self.client_secret.as_str())
                .append_pair("code", &code)
                .append_pair("state", &state)
                .finish();
            req =<< future::result(ClientRequest::post(self.access_token_endpoint.as_str())
                .with_connector(self.connector.clone())
                .header("content-type", "application/x-www-form-urlencoded")
                .header("accept", "application/json")
                .body(encoded)
                .map_err(ErrorInternalServerError)
            );
            resp =<< req.send().map_err(ErrorInternalServerError);
            token =<< resp.json::<AccessToken>().map_err(ErrorInternalServerError);
            ret ret(token)
        }
    }
}

impl<S> Middleware<S> for OAuth2Middleware {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started, ::actix_web::Error> {
        match (req.method(), req.path()) {
            (&Method::GET, path) |
            (&Method::POST, path) if path == self.login_path.as_str() => { return self.handle_login(req) },
            (&Method::GET, path) if path == self.callback_path.as_str() => { return self.handle_callback(req); },
            _ => { return Ok(Started::Done); }
        }
    }
}

mod github_api {
    use super::*;
    #[derive(Serialize, Deserialize, Debug)]
    pub struct GitHubUserData {
        pub id: u64,
        pub login: String,
    }
    pub fn get_user_data(conn: ::actix::Addr<::actix_web::client::ClientConnector>, token: &AccessToken) -> impl Future<Item=GitHubUserData, Error=::actix_web::Error> {
        mdo!{
            // https://developer.github.com/v3/users/#get-the-authenticated-user
            req =<< future::result(ClientRequest::post("https://api.github.com/user")
                .with_connector(conn)
                .header("authorization", format!("{} {}", token.token_type, token.access_token))
                .header("content-type", "application/json")
                .header("accept", "application/json")
                .body("{}")
            );
            resp =<< req.send().from_err();
            // data =<< resp.body().from_err();
            // data =<< future::result(std::str::from_utf8(&data).map(|s|s.to_owned())).from_err();
            data =<< resp.json::<GitHubUserData>().from_err();
            ret ret(data)
        }
    }
}