#![feature(backtrace)]

#[derive(Debug, Clone)]
struct ErrorLogger {}

impl<S, B> actix_web::dev::Transform<S> for ErrorLogger
where
    S: actix_web::dev::Service<Request = actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error>
        + 'static,
    S::Future: 'static,
    B: actix_web::dev::MessageBody + 'static,
{
    type Request = actix_web::dev::ServiceRequest;
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = ErrorLoggerMiddleware<S>;
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(ErrorLoggerMiddleware {
            service: std::rc::Rc::new(std::cell::RefCell::new(service)),
        }))
    }
}
pub struct ErrorLoggerMiddleware<S> {
    service: std::rc::Rc<std::cell::RefCell<S>>,
}

impl<S, B> actix_web::dev::Service for ErrorLoggerMiddleware<S>
where
    S: actix_web::dev::Service<Request = Self::Request, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error>
        + 'static,
    S::Future: 'static,
    B: actix_web::dev::MessageBody + 'static,
{
    type Request = actix_web::dev::ServiceRequest;
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output=Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&mut self, ctx: &mut std::task::Context) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let mut service = self.service.clone();
        Box::pin(async move {
            // dbg!(&req);
            let res = service.call(req).await.map_err(|err|{
                // dbg!(&err);
                err
            })?;
            // dbg!(&res);
            if let Some(err) = res.response().error(){
                if let Some(err) = err.as_error::<MyError>() {
                    use std::error::Error;
                    println!("error: {}", err);
                    println!("error: {:?}", err);

                    let mut source = std::error::Error::source(err);
                    while let Some(err) = source {
                        println!("caused by: {}", err);
                        println!("caused by: {:?}", err);
                        source = std::error::Error::source(err);
                    }
                    if let Some(bt) = err.backtrace() {
                        println!("{}", bt);
                        println!("{:?}", bt);
                    }
                }
            }
            Ok(res)
        })
    }
}
#[derive(err_derive::Error, Debug)]
#[error(display="my error, {}", err)]
struct MyError{
    #[error(source, from)]
    err: MyServiceError,
}

#[derive(err_derive::Error, Debug)]
#[error(display="my servie error")]
struct MyServiceError{
    #[error(source, from)]
    err: std::io::Error,
}
impl actix_web::error::ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode{
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::dev::Body>{
        actix_web::HttpResponse::InternalServerError().json2(&serde_json::json!({}))
    }   
}
#[actix_web::get("/{id}/{name}/index.html")]
async fn index(actix_web::web::Path((_id, _name)): actix_web::web::Path<(u32, String)>) -> Result<String, MyError> {
    Err(MyError{err:MyServiceError{err: std::io::ErrorKind::NotFound.into()}})
    // Ok(format!("Hello {}! id:{}", name, id))
}
fn a(_: impl actix_web::ResponseError){}
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let err = a(Box::new(MyError{err:MyServiceError{err: std::io::ErrorKind::NotFound.into()}}));
    println!("{:?}", err);
    // env_logger::init();
    // actix_web::HttpServer::new(||
    //     actix_web::App::new()
    //         // .wrap(actix_web::middleware::Logger::default())
    //         .wrap(ErrorLogger{})
    //         .service(index)
    // )
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await?;
    Ok(())
}
