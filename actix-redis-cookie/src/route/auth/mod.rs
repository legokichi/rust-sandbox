use super::*;

pub mod github;

pub fn login(_req: HttpRequest<Ctx>) -> Result<HttpResponse, ::actix_web::Error> {
    let body = format!(r##"<!DOCTYPE html>
<html>
<head>
<title>login</title>
</head>
<body>
<form action="/auth/sign/github" method="post">
<input type="submit" value="login with github" />
</form>
</body>
</html>"##);
    Ok(HttpResponse::Ok().body(body))
}

pub fn logout(req: HttpRequest<Ctx>) -> Result<HttpResponse, ::actix_web::Error> {
    req.session().clear();
    Ok(HttpResponse::SeeOther().header("Location", "/").finish())
}