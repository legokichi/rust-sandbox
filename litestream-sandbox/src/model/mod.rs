mod point;
pub use point::*;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct List<T> {
    pub offset: Option<u32>,
    pub list: T,
}