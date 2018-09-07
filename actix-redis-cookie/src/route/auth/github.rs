use super::*;

/// POST /auth/sign/github
pub fn post_auth_sign_github(req: HttpRequest<Ctx>) -> Result<HttpResponse, ::actix_web::Error> {
    let mut url = ::url::Url::parse("https://github.com/login/oauth/authorize")?;
    url.query_pairs_mut()
        .append_pair("client_id", &req.state().client_id)
        .append_pair("scope", "user") // user:email
        .append_pair("state", "1234")
        .append_pair("redirect_uri", "https://localhost:8080/auth/cb/github");
    let resp = HttpResponse::Found().header("Location", url.as_str()).finish();
    Ok(resp)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CallbackResp {
    Success{ code: String, state: String },
    Error{ error: String, error_description: String, error_uri: String, state: String }
}
#[derive(Serialize, Deserialize, Debug)]
struct AccessToken { access_token: String, token_type: String }
#[derive(Serialize, Deserialize, Debug)]
struct GitHubUserData { id: u64, login: String }
pub fn get_auth_cb_github((req, ctx, query): (HttpRequest<Ctx>, State<Ctx>, Query<CallbackResp>)) -> Box<dyn Future<Item=HttpResponse, Error=::actix_web::Error>> {
    let ( code, state ) = match query.into_inner() {
        CallbackResp::Error{ error, error_description, error_uri, .. } => {
            return Box::new(future::ok(HttpResponse::InternalServerError().body(format!(r##"{} {} {}"##, error, error_description, error_uri))));
        },
        CallbackResp::Success{ code, state } => ( code, state ),
    };
    let connector = ctx.conn.to_owned();
    let connector2 = ctx.conn.to_owned();
    let session = req.session();
    let encoded: String = ::url::form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", &ctx.client_id)
        .append_pair("client_secret", &ctx.client_secret)
        .append_pair("code", &code)
        .append_pair("state", &state)
        .finish();
    let fut = mdo!{
        req =<< future::result(ClientRequest::post("https://github.com/login/oauth/access_token")
            .with_connector(connector)
            .header("content-type", "application/x-www-form-urlencoded")
            .header("accept", "application/json")
            .body(encoded)
        ).map_err(ErrorInternalServerError);
        resp =<< req.send().from_err();
        token =<< resp.json::<AccessToken>().from_err();
        let _ = println!("{:?}", token);
        // https://developer.github.com/v3/users/#get-the-authenticated-user
        req =<< future::result(ClientRequest::post("https://api.github.com/user")
            .with_connector(connector2)
            .header("authorization", format!("{} {}", token.token_type, token.access_token))
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .body("{}")
        ).from_err();
        resp =<< req.send().from_err();
        // data =<< resp.body().from_err();
        // data =<< future::result(std::str::from_utf8(&data).map(|s|s.to_owned())).from_err();
        data =<< resp.json::<GitHubUserData>().from_err();
        let _ = debug!("{:?}", data);
        () =<< future::result(session.set::<String>("username", data.login)).from_err();
        ret ret(HttpResponse::SeeOther().header("Location", "/").finish())
    };
    fut.responder()
}
