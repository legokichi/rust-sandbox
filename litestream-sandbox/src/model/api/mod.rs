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
pub enum Request {
    ListUser(crate::model::api::list_user::Request),
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
pub enum Response {
    ListUser(crate::model::api::list_user::Response),
}

pub mod list_user {
    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
    pub struct Request {
        pub offset: Option<u32>,
        pub limit: Option<u32>,
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
    pub struct Response {
        pub users: Vec<crate::model::user::User>,
        pub next: u32,
    }
}
