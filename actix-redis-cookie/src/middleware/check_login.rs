use super::*;

pub struct CheckLoginMiddleware {
    session_key: String,
    /// None => 401 UnAuthorized(default)
    /// Some("/") => 303 SeeOther Location: /
    redirect_path: Option<String>,
}

impl Default for CheckLoginMiddleware {
    fn default() -> CheckLoginMiddleware {
        CheckLoginMiddleware {
            session_key: "login_session".into(),
            redirect_path: None,
        }
    }
}

impl<S> Middleware<S> for CheckLoginMiddleware {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started, ::actix_web::Error> {
        if let Ok(Some(_)) = req.session().get::<String>(&self.session_key) {
            return Ok(Started::Done);
        }
        req.session().clear();
        if let &Some(ref redirect_path) = &self.redirect_path {
            let resp = HttpResponse::SeeOther().header("location", redirect_path.as_str()).finish();
            return Ok(Started::Response(resp));
        }
        return Err(ErrorUnauthorized("401 Unauthorized"));
    }
}
