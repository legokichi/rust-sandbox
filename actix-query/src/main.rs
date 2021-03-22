use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, Debug)]
struct A{
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    b: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    c: Option<bool>,
    #[serde(with = "::chrono::serde::ts_milliseconds_option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    d: Option<DateTime<Utc>>,
}
fn main() {
    use actix_web::web::Query;
    dbg!(actix_web::web::Query::<A>::from_query("a=a&b=0&c=true&d=null"));
}
