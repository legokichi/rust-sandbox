use diesel::result::Error as QueryResultError;
use diesel::ConnectionError;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "diesel connection error")]
    Connection,
    #[fail(display = "diesel query result error")]
    Query,
}

impl From<ConnectionError> for Error {
    fn from(error: ConnectionError) -> Error {
        Error {
            inner: error.context(ErrorKind::Connection),
        }
    }
}

impl From<QueryResultError> for Error {
    fn from(error: QueryResultError) -> Error {
        Error {
            inner: error.context(ErrorKind::Query),
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
