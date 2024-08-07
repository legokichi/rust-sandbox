#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Eq,
    derive_more::TryInto,
    derive_more::From,
)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Request {
    ListUser(crate::model::api::list_user::Request),
    ListAccessLogs(crate::model::api::list_access_logs::Request),
}

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    Clone,
    PartialEq,
    Eq,
    derive_more::TryInto,
    derive_more::From,
)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Response {
    ListUser(crate::model::api::list_user::Response),
    ListAccessLogs(crate::model::api::list_access_logs::Response),
    Error(ErrorKind),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(tag = "errorType")]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    PermissionDenied,
    InvalidRequest,
}

impl Request {
    pub fn to_request_type(&self) -> Result<String, anyhow::Error> {
        use anyhow::Context;
        let json = serde_json::to_value(self)?;
        let json = json.pointer(".type").context("type field not found")?;
        let txt = json.as_str().context("type field is not a string")?;
        Ok(txt.to_string())
    }
}

impl Response {
    pub fn to_response_type(&self) -> Result<String, anyhow::Error> {
        use anyhow::Context;
        let json = serde_json::to_value(self)?;
        let json = json.pointer(".type").context("type field not found")?;
        let txt = json.as_str().context("type field is not a string")?;
        Ok(txt.to_string())
    }
}

pub mod list_user {
    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct Request {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u32>,
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub users: Vec<crate::model::user::User>,
        pub next: u32,
    }
}

pub mod list_access_logs {
    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct Request {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub offset: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_id: Option<i64>,
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub access_logs: Vec<crate::model::user::AccessLog>,
        pub next: u32,
    }
}
