use service::Error as ServiceError;
use serde_urlencoded::de::Error as UrlParseError;
use askama::Error as AskamaError;
use failure::SyncFailure;
use std::io::Error as IOError;
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::{Response, StatusCode};
use hyper::{Body, Error as HyperError, StatusCode as HyperStatusCode};

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "IO error")]
    Io,
    #[fail(display = "Service error: [{}] {}", _0, _1)]
    ServiceError(HyperStatusCode, String),
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

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error {
            inner: error.context(ErrorKind::Io),
        }
    }
}

impl<'a> From<(HyperStatusCode, &'a [u8])> for Error {
    fn from(err: (HyperStatusCode, &'a [u8])) -> Self {
        let (status_code, msg) = err;
        Error::from(ErrorKind::ServiceError(
            status_code,
            ::std::str::from_utf8(msg)
                .unwrap_or_else(|_| "Could not decode error message")
                .to_string(),
        ))
    }
}

impl From<ServiceError> for Error {
    fn from(error: ServiceError) -> Error {
        Error {
            inner: error.context(ErrorKind::Service),
        }
    }
}

impl From<UrlParseError> for Error {
    fn from(error: UrlParseError) -> Error {
        Error {
            inner: error.context(ErrorKind::UrlParse),
        }
    }
}

impl From<HyperError> for Error {
    fn from(error: HyperError) -> Error {
        Error {
            inner: error.context(ErrorKind::Hyper),
        }
    }
}

impl From<SyncFailure<AskamaError>> for Error {
    fn from(error: SyncFailure<AskamaError>) -> Error {
        Error {
            inner: error.context(ErrorKind::Askama),
        }
    }
}

/* ----------- failure crate template ----------- */

use std::fmt;
use std::fmt::Display;
use failure::{Backtrace, Context, Fail};

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
