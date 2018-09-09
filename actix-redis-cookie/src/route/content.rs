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
<script>
const ws = new WebSocket("wss://localhost:8080/ws");
ws.addEventListener("error", console.error.bind(console));
ws.addEventListener("close", console.log.bind(console));
ws.addEventListener("open", (ev)=>{{
    console.log(ev);
    ws.addEventListener("message", console.log.bind(console));
    ws.send("hello");
}});
</script>
</html>"##, username);
    Ok(HttpResponse::Ok().body(body))
}