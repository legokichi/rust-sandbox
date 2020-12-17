#![feature(prelude_import)]
#![feature(backtrace)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
struct ErrorLogger {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for ErrorLogger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            ErrorLogger {} => {
                let mut debug_trait_builder = f.debug_struct("ErrorLogger");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for ErrorLogger {
    #[inline]
    fn clone(&self) -> ErrorLogger {
        match *self {
            ErrorLogger {} => ErrorLogger {},
        }
    }
}
impl<S, B> actix_web::dev::Transform<S> for ErrorLogger
where
    S: actix_web::dev::Service<
            Request = actix_web::dev::ServiceRequest,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        > + 'static,
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
    S: actix_web::dev::Service<
            Request = Self::Request,
            Response = actix_web::dev::ServiceResponse<B>,
            Error = actix_web::Error,
        > + 'static,
    S::Future: 'static,
    B: actix_web::dev::MessageBody + 'static,
{
    type Request = actix_web::dev::ServiceRequest;
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + 'static>,
    >;
    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
    fn call(&mut self, req: Self::Request) -> Self::Future {
        let mut service = self.service.clone();
        Box::pin(async move {
            let res = service.call(req).await.map_err(|err| err)?;
            if let Some(err) = res.response().error() {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["error: ", "\n"],
                        &match (&err,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                };
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["error: ", "\n"],
                        &match (&err,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        },
                    ));
                };
                let mut source = std::error::Error::source(err);
                while let Some(err) = source {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["caused by: ", "\n"],
                            &match (&err,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                    };
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["caused by: ", "\n"],
                            &match (&err,) {
                                (arg0,) => {
                                    [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                                }
                            },
                        ));
                    };
                    source = std::error::Error::source(err);
                }
                if let Some(bt) = std::error::Error::backtrace(err) {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["", "\n"],
                            &match (&bt,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                    };
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["", "\n"],
                            &match (&bt,) {
                                (arg0,) => {
                                    [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                                }
                            },
                        ));
                    };
                }
            }
            Ok(res)
        })
    }
}
#[error(display = "my error, {}", err)]
struct MyError {
    #[error(source, from)]
    err: MyServiceError,
}
#[allow(non_upper_case_globals)]
#[doc(hidden)]
const _DERIVE_std_error_Error_FOR_MyError: () = {
    impl ::std::error::Error for MyError {
        fn description(&self) -> &str {
            "description() is deprecated; use Display"
        }
        #[allow(unreachable_code)]
        fn cause(&self) -> ::std::option::Option<&::std::error::Error> {
            match *self {
                MyError {
                    err: ref __binding_0,
                } => return Some(__binding_0 as &::std::error::Error),
            }
            None
        }
        #[allow(unreachable_code)]
        fn source(&self) -> ::std::option::Option<&(::std::error::Error + 'static)> {
            match *self {
                MyError {
                    err: ref __binding_0,
                } => return Some(__binding_0 as &::std::error::Error),
            }
            None
        }
    }
};
#[allow(non_upper_case_globals)]
#[doc(hidden)]
const _DERIVE_core_fmt_Display_FOR_MyError: () = {
    impl ::core::fmt::Display for MyError {
        #[allow(unreachable_code)]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MyError {
                    err: ref __binding_0,
                } => {
                    return f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["my error, "],
                        &match (&__binding_0,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["An error has occurred."],
                &match () {
                    () => [],
                },
            ))
        }
    }
};
#[allow(non_upper_case_globals)]
#[doc(hidden)]
const _DERIVE_core_convert_From_MyServiceError_FOR_MyError: () = {
    impl ::core::convert::From<MyServiceError> for MyError {
        fn from(from: MyServiceError) -> Self {
            MyError { err: from }
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MyError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            MyError {
                err: ref __self_0_0,
            } => {
                let mut debug_trait_builder = f.debug_struct("MyError");
                let _ = debug_trait_builder.field("err", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[error(display = "my servie error")]
struct MyServiceError {
    #[error(source, from)]
    err: std::io::Error,
}
#[allow(non_upper_case_globals)]
#[doc(hidden)]
const _DERIVE_std_error_Error_FOR_MyServiceError: () = {
    impl ::std::error::Error for MyServiceError {
        fn description(&self) -> &str {
            "description() is deprecated; use Display"
        }
        #[allow(unreachable_code)]
        fn cause(&self) -> ::std::option::Option<&::std::error::Error> {
            match *self {
                MyServiceError {
                    err: ref __binding_0,
                } => return Some(__binding_0 as &::std::error::Error),
            }
            None
        }
        #[allow(unreachable_code)]
        fn source(&self) -> ::std::option::Option<&(::std::error::Error + 'static)> {
            match *self {
                MyServiceError {
                    err: ref __binding_0,
                } => return Some(__binding_0 as &::std::error::Error),
            }
            None
        }
    }
};
#[allow(non_upper_case_globals)]
#[doc(hidden)]
const _DERIVE_core_fmt_Display_FOR_MyServiceError: () = {
    impl ::core::fmt::Display for MyServiceError {
        #[allow(unreachable_code)]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                MyServiceError {
                    err: ref __binding_0,
                } => {
                    return f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["my servie error"],
                        &match () {
                            () => [],
                        },
                    ))
                }
            }
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["An error has occurred."],
                &match () {
                    () => [],
                },
            ))
        }
    }
};
#[allow(non_upper_case_globals)]
#[doc(hidden)]
const _DERIVE_core_convert_From_std_io_Error_FOR_MyServiceError: () = {
    impl ::core::convert::From<std::io::Error> for MyServiceError {
        fn from(from: std::io::Error) -> Self {
            MyServiceError { err: from }
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for MyServiceError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            MyServiceError {
                err: ref __self_0_0,
            } => {
                let mut debug_trait_builder = f.debug_struct("MyServiceError");
                let _ = debug_trait_builder.field("err", &&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl actix_web::error::ResponseError for MyError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::dev::Body> {
        actix_web::HttpResponse::InternalServerError()
            .json2(&::serde_json::Value::Object(::serde_json::Map::new()))
    }
}
#[allow(non_camel_case_types, missing_docs)]
pub struct index;
impl actix_web::dev::HttpServiceFactory for index {
    fn register(self, __config: &mut actix_web::dev::AppService) {
        async fn index(
            actix_web::web::Path((_id, _name)): actix_web::web::Path<(u32, String)>,
        ) -> Result<String, MyError> {
            Err(MyError {
                err: MyServiceError {
                    err: std::io::ErrorKind::NotFound.into(),
                },
            })
        }
        let __resource = actix_web::Resource::new("/{id}/{name}/index.html")
            .name("index")
            .guard(actix_web::guard::Get())
            .to(index);
        actix_web::dev::HttpServiceFactory::register(__resource, __config)
    }
}
fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    actix_web::rt::System::new("main").block_on(async move {
        {
            env_logger::init();
            actix_web::HttpServer::new(|| {
                actix_web::App::new()
                    .wrap(actix_web::middleware::Logger::default())
                    .wrap(ErrorLogger {})
                    .service(index)
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await?;
            Ok(())
        }
    })
}
