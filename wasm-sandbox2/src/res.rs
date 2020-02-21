use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Response<T, E> {
    Ok(T),
    Error(E),
}

impl<T, E> From<Result<T, E>> for Response<T, E> {
    fn from(o: Result<T, E>) -> Self {
        match o {
            Ok(o) => Self::Ok(o),
            Err(o) => Self::Error(o),
        }
    }
}

impl<T, E> Into<Result<T, E>> for Response<T, E> {
    fn into(self) -> Result<T, E> {
        match self {
            Response::Ok(o) => Ok(o),
            Response::Error(o) => Err(o),
        }
    }
}
