// pub mod point;
pub mod user;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
pub struct List<T> {
    pub offset: Option<u32>,
    pub list: T,
}
