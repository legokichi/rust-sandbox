use super::*;

pub fn index(_req: HttpRequest<Ctx>, username: String) -> Result<HttpResponse, ::actix_web::Error> {
    let body = format!(r##"<!DOCTYPE html>
<head>
<title>logined</title>
</head>
hello {}!
<form action="/logout" method="post">
<input type="submit" value="logout" />
</form>
</html>"##, username);
    Ok(HttpResponse::Ok().body(body))
}