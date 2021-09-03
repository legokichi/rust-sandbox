/// hoge_service::handler が返す型
#[derive(thiserror::Error, Debug)]
#[error("SvcError: {}", retriable)]
pub struct SvcError {
    pub retriable: bool,
    #[source]
    pub error: anyhow::Error,
}

impl SvcError {
    pub fn should_not_retry(err: impl Into<anyhow::Error>) -> Self {
        Self {
            retriable: false,
            error: err.into(),
        }
    }
    pub fn should_retry(err: impl Into<anyhow::Error>) -> Self {
        Self {
            retriable: true,
            error: err.into(),
        }
    }
    fn context<C>(self, ctx: C) -> Self
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        if self.retriable {
            Self::should_retry(self.error.context(ctx))
        } else {
            Self::should_not_retry(self.error.context(ctx))
        }
    }
}

use std::fmt::Display;
pub trait Context<T, E> {
    fn context<C>(self, context: C) -> Result<T, SvcError>
    where
        C: Display + Send + Sync + 'static;
}

impl<T, E> Context<T, E> for Result<T, E>
where
    E: Into<SvcError> + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, SvcError>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|err| Into::<SvcError>::into(err).context(context))
    }
}

impl<T> Context<T, std::convert::Infallible> for Option<T> {
    fn context<C>(self, context: C) -> Result<T, SvcError>
    where
        C: Display + Send + Sync + 'static,
    {
        self.ok_or_else(|| SvcError::should_not_retry(anyhow::anyhow!("{}", context)))
    }
}

impl From<crate::kinesis::KinesisDecodeError> for SvcError {
    fn from(e: crate::kinesis::KinesisDecodeError) -> Self {
        use crate::kinesis::KinesisDecodeError::*;

        match e {
            e @ Serde(..) => SvcError::should_not_retry(e),
        }
    }
}

impl From<crate::kinesis::MqttPayloadEncodeError> for SvcError {
    fn from(e: crate::kinesis::MqttPayloadEncodeError) -> Self {
        use crate::kinesis::MqttPayloadEncodeError::*;

        match e {
            e @ CompressFailed(..) => SvcError::should_not_retry(e),
        }
    }
}

impl From<crate::kinesis::MqttPayloadDecodeError> for SvcError {
    fn from(e: crate::kinesis::MqttPayloadDecodeError) -> Self {
        use crate::kinesis::MqttPayloadDecodeError::*;

        match e {
            e @ DecompressFailed(..) => SvcError::should_not_retry(e),
            e @ DeserializeFailed(..) => SvcError::should_not_retry(e),
            e @ InvalidMsgpack(..) => SvcError::should_not_retry(e),
        }
    }
}

impl From<crate::dynamodb::IntoAttrValError> for SvcError {
    fn from(e: crate::dynamodb::IntoAttrValError) -> Self {
        use crate::dynamodb::IntoAttrValError::*;

        match e {
            e @ IntoJsonError(..) => SvcError::should_not_retry(e),
            e @ NotObject(..) => SvcError::should_not_retry(e),
        }
    }
}
impl From<crate::dynamodb::FromAttrValError> for SvcError {
    fn from(e: crate::dynamodb::FromAttrValError) -> Self {
        use crate::dynamodb::FromAttrValError::*;

        match e {
            e @ FromJsonError(..) => SvcError::should_not_retry(e),
            e @ UnsupportedAttrValType(..) => SvcError::should_not_retry(e),
            e @ CannotParseNumber(..) => SvcError::should_not_retry(e),
        }
    }
}

impl From<crate::client::ClientError> for SvcError {
    fn from(e: crate::client::ClientError) -> Self {
        if e.retriable {
            SvcError::should_retry(e)
        } else {
            SvcError::should_not_retry(e)
        }
    }
}
