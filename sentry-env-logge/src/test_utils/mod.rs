pub use assert_json_diff::{assert_json_eq, assert_json_matches_no_panic};
pub use mockall::predicate::*;
pub use proptest::prelude::*;
pub use proptest_derive::Arbitrary;
pub use rstest::*;

pub mod device;
pub mod device_api;

proptest! {
    #[test]
    fn datetime(timestamp in -8334632851200000_i64..=8210298412799999_i64) {
        use chrono::TimeZone;
        chrono::Utc.timestamp_millis_opt(timestamp).unwrap();
    }
}

prop_compose! {
    // https://github.com/chronotope/chrono/pull/258#discussion_r201495430
    // chrono::serde は負の timestamp を扱えないらしい
    pub fn arb_datetime()
        (timestamp in 0_i64..=8210298412799999_i64)
        -> chrono::DateTime<chrono::Utc> {
        use chrono::TimeZone;
        chrono::Utc.timestamp_millis_opt(timestamp).unwrap()
    }
}

pub fn arb_uuid() -> impl Strategy<Value = uuid::Uuid> {
    Just(::uuid::Uuid::new_v4())
}

pub fn arb_url() -> impl Strategy<Value = url::Url> {
    Just("https://example.com/".parse().unwrap())
}

pub fn arb_ip() -> impl Strategy<Value = ipnetwork::IpNetwork> {
    use ipnetwork::IpNetwork;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
    prop_oneof![
        any::<(u8, u8, u8, u8)>()
            .prop_map(|(a, b, c, d)| IpNetwork::from(IpAddr::from(Ipv4Addr::new(a, b, c, d)))),
        any::<(u16, u16, u16, u16, u16, u16, u16, u16)>().prop_map(|(a, b, c, d, e, f, g, h)| {
            IpNetwork::from(IpAddr::from(Ipv6Addr::new(a, b, c, d, e, f, g, h)))
        }),
    ]
}

pub fn arb_json() -> impl Strategy<Value = serde_json::Value> {
    let leaf = prop_oneof![
        Just(serde_json::Value::Null),
        any::<bool>().prop_map(|o| serde_json::to_value(o).unwrap()),
        any::<f64>().prop_map(|o| serde_json::to_value(o).unwrap()),
        ".*".prop_map(|o| serde_json::to_value(o).unwrap()),
    ];
    leaf.prop_recursive(
        4,   // 4 levels deep
        128, // Shoot for maximum size of 128 nodes
        5,   // We put up to 5 items per collection
        |inner| {
            prop_oneof![
                // Take the inner strategy and make the two recursive cases.
                prop::collection::vec(inner.clone(), 0..5)
                    .prop_map(|o| serde_json::to_value(o).unwrap()),
                prop::collection::hash_map(".*", inner, 0..5)
                    .prop_map(|o| serde_json::to_value(o).unwrap()),
            ]
        },
    )
}

pub fn arb_json_object() -> impl Strategy<Value = serde_json::Value> {
    prop::collection::hash_map(".*", arb_json(), 0..5)
        .prop_map(|o| serde_json::to_value(o).unwrap())
}

pub fn arb_js_option<T: Arbitrary + Clone>() -> impl Strategy<Value = js_option::JsOption<T>> {
    prop_oneof![
        T::arbitrary().prop_map(js_option::JsOption::Some),
        Just(js_option::JsOption::Null),
        Just(js_option::JsOption::Undefined),
    ]
}
