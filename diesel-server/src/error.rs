use askama::Error as AskamaError;
use failure::SyncFailure;
use actix_web::error::Error as ActixError;
use service::Error as ServiceError;
use std::io::Error as IOError;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "IO error")]
    Io,
    #[fail(display = "Service error: [{}] {}", _0, _1)]
    ServiceError(ActixError, String),
    #[fail(display = "service error")]
    Service,
    #[fail(display = "Url parse error")]
    Parse,
    #[fail(display = "Serde error")]
    Serde,
    #[fail(display = "Invalid uri {}", _0)]
    InvalidUri(String),
    #[fail(display = "Cannot parse uri")]
    UrlParse,
    #[fail(display = "Hyper error")]
    Hyper,
    #[fail(display = "askama error")]
    Askama,
    #[fail(display = "Module not found")]
    NotFound,
}

impl Into<ActixError> for Error {
    fn into(self) -> ActixError {
        let mut fail: &Fail = &self;
        let mut message = self.to_string();
        while let Some(cause) = fail.cause() {
            message.push_str(&format!("\n\tcaused by: {}", cause.to_string()));
            fail = cause;
        }
        match *self.kind() {
            _ => ::actix_web::error::ErrorInternalServerError(message),
        }
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error {
            inner: error.context(ErrorKind::Io),
        }
    }
}

impl From<ServiceError> for Error {
    fn from(error: ServiceError) -> Error {
        Error {
            inner: error.context(ErrorKind::Service),
        }
    }
}

impl From<AskamaError> for Error {
    fn from(error: AskamaError) -> Error {
        Error {
            inner: SyncFailure::new(error).context(ErrorKind::Askama),
        }
    }
}

// ----------- failure crate template -----------

use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}
