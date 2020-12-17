use rusoto_dynamodb::AttributeValue;
// use serde::Deserialize;
use std::collections::HashMap;

// #[derive(Deserialize)]
// #[serde(tag="type")]
pub enum EventType {
    Connected,
    Disconnected,
}
// #[doc(hidden)]
// #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    // #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    // #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for EventType {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 2",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Connected" => _serde::export::Ok(__Field::__field0),
                        "Disconnected" => _serde::export::Ok(__Field::__field1),
                        _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Connected" => _serde::export::Ok(__Field::__field0),
                        b"Disconnected" => _serde::export::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::export::from_utf8_lossy(__value);
                            _serde::export::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            const VARIANTS: &'static [&'static str] = &["Connected", "Disconnected"];
            let __tagged = match _serde::Deserializer::deserialize_any(
                __deserializer,
                _serde::private::de::TaggedContentVisitor::<__Field>::new("type"),
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match __tagged.tag {
                __Field::__field0 => {
                    match _serde::Deserializer::deserialize_any(
                        _serde::private::de::ContentDeserializer::<__D::Error>::new(
                            __tagged.content,
                        ),
                        _serde::private::de::InternallyTaggedUnitVisitor::new(
                            "EventType",
                            "Connected",
                        ),
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::export::Ok(EventType::Connected)
                }
                __Field::__field1 => {
                    match _serde::Deserializer::deserialize_any(
                        _serde::private::de::ContentDeserializer::<__D::Error>::new(
                            __tagged.content,
                        ),
                        _serde::private::de::InternallyTaggedUnitVisitor::new(
                            "EventType",
                            "Disconnected",
                        ),
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::export::Ok(EventType::Disconnected)
                }
            }
        }
    }
};

fn main() {
    let hashmap = vec![
        ("type".to_string(), AttributeValue{s: Some("connected".to_string()), ..Default::default()})
    ].into_iter().collect::<HashMap<String, AttributeValue>>();
    let _: EventType = serde_dynamodb::from_hashmap(hashmap).unwrap();
}
