use super::*;

pub fn index(_req: HttpRequest<Ctx>, username: String) -> Box<dyn Future<Item=HttpResponse, Error=::actix_web::Error>> {
    let body = format!(r##"<!DOCTYPE html>
<head>
<title>logined</title>
</head>
hello {}!
<form action="/auth/logout" method="post">
<input type="submit" value="logout" />
</form>
</html>"##, username);
    Box::new(future::result(Ok(HttpResponse::Ok().body(body))))
}