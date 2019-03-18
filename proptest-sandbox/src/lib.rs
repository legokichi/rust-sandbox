#![allow(clippy::unit_arg)]

use base64::STANDARD;
use base64_serde::base64_serde_type;
use chrono::prelude::*;
use proptest::prelude::*;
use proptest_derive::*;
use regex::Regex;
use serde_derive::*;
use url::Url;
use uuid::Uuid;

base64_serde_type!(Base64Standard, STANDARD);

fn arb_datetime() -> impl Strategy<Value = ::chrono::DateTime<::chrono::Utc>> {
    Just(::chrono::Utc::now())
}

fn arb_url() -> impl Strategy<Value = ::url::Url> {
    Just("https://example.com/".parse().unwrap())
}

fn arb_uuid() -> impl Strategy<Value = ::uuid::Uuid> {
    Just(::uuid::Uuid::new_v4())
}

fn arb_json(depth: u32) -> impl Strategy<Value = ::serde_json::Value> {
    let leaf = prop_oneof![
        Just(::serde_json::Value::Null),
        any::<bool>().prop_map(|o| serde_json::to_value(o).unwrap()),
        any::<f64>().prop_map(|o| serde_json::to_value(o).unwrap()),
        ".*".prop_map(|o| serde_json::to_value(o).unwrap()),
    ];
    leaf.prop_recursive(
        depth, // n levels deep
        256,   // Shoot for maximum size of 256 nodes
        10,    // We put up to 10 items per collection
        |inner| {
            prop_oneof![
                // Take the inner strategy and make the two recursive cases.
                prop::collection::vec(inner.clone(), 0..10)
                    .prop_map(|o| serde_json::to_value(o).unwrap()),
                prop::collection::hash_map(".*", inner, 0..10)
                    .prop_map(|o| serde_json::to_value(o).unwrap()),
            ]
        },
    )
}
fn arb_regex() -> impl Strategy<Value = ::regex::Regex> {
    Just(Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap())
}

#[derive(Arbitrary, Serialize, Deserialize, Clone, Debug)]
struct Foo {
    pub bar: Bar,
    #[proptest(strategy = "crate::arb_datetime()")]
    pub rfc3339: DateTime<Utc>,
    #[proptest(strategy = "crate::arb_datetime()")]
    #[serde(with = "::chrono::serde::ts_milliseconds")]
    pub unix_millis: DateTime<Utc>,
    #[proptest(strategy = "crate::arb_datetime()")]
    #[serde(with = "::chrono::serde::ts_seconds")]
    pub unix_micros: DateTime<Utc>,
    #[proptest(strategy = "crate::arb_url()")]
    #[serde(with = "::url_serde")]
    pub url: Url,
    #[proptest(strategy = "crate::arb_uuid()")]
    pub uuid: Uuid,
    #[proptest(strategy = "prop::collection::vec(proptest::num::u8::ANY, 0..2)")]
    #[serde(with = "Base64Standard")]
    pub buffer: Vec<u8>,
    #[proptest(strategy = "crate::arb_json(4)")]
    pub json: serde_json::Value,
    #[proptest(strategy = "prop::collection::vec(crate::arb_json(4), 0..2)")]
    pub jsons: Vec<serde_json::Value>,
    #[proptest(strategy = "crate::arb_regex()")]
    #[serde(with = "::serde_regex")]
    pub pattern: Regex,
}

#[derive(Arbitrary, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum Bar {
    A(String),
    B(u8),
    C((u8, u16)),
    D(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        proptest!(|(foo: Foo)| {
            dbg!(&foo);
            let foo_json = serde_json::to_string_pretty(&foo).unwrap();
            println!("{}", &foo_json);
        });
    }
}
