#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use serde::{Serialize, Deserialize};
#[serde(rename = "boards")]
pub enum Boards {
    #[serde(rename = "RSPi1A")]
    Rspi1A,
    #[serde(rename = "RSPi1B")]
    Rspi1B,
    #[serde(rename = "RSPi1APlus")]
    Rspi1APlus,
    #[serde(rename = "RSPi1BPlus")]
    Rspi1BPlus,
    #[serde(rename = "RSPi2B")]
    Rspi2B,
    #[serde(rename = "RSPiAlpha")]
    RspiAlpha,
    #[serde(rename = "RSPiCM1")]
    RspiCM1,
    #[serde(rename = "RSPi3B")]
    Rspi3B,
    #[serde(rename = "RSPiZero")]
    RspiZero,
    #[serde(rename = "RSPiCM3")]
    RspiCM3,
    #[serde(rename = "RSPiZeroW")]
    RspiZeroW,
    #[serde(rename = "RSPi3BPlus")]
    Rspi3BPlus,
    #[serde(rename = "RSPi3APlus")]
    Rspi3APlus,
    #[serde(rename = "RSPiCM3Plus")]
    RspiCM3Plus,
    #[serde(rename = "RSPi4B")]
    Rspi4B,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Boards {
    #[inline]
    fn clone(&self) -> Boards {
        match (&*self,) {
            (&Boards::Rspi1A,) => Boards::Rspi1A,
            (&Boards::Rspi1B,) => Boards::Rspi1B,
            (&Boards::Rspi1APlus,) => Boards::Rspi1APlus,
            (&Boards::Rspi1BPlus,) => Boards::Rspi1BPlus,
            (&Boards::Rspi2B,) => Boards::Rspi2B,
            (&Boards::RspiAlpha,) => Boards::RspiAlpha,
            (&Boards::RspiCM1,) => Boards::RspiCM1,
            (&Boards::Rspi3B,) => Boards::Rspi3B,
            (&Boards::RspiZero,) => Boards::RspiZero,
            (&Boards::RspiCM3,) => Boards::RspiCM3,
            (&Boards::RspiZeroW,) => Boards::RspiZeroW,
            (&Boards::Rspi3BPlus,) => Boards::Rspi3BPlus,
            (&Boards::Rspi3APlus,) => Boards::Rspi3APlus,
            (&Boards::RspiCM3Plus,) => Boards::RspiCM3Plus,
            (&Boards::Rspi4B,) => Boards::Rspi4B,
        }
    }
}
impl ::core::marker::StructuralPartialEq for Boards {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Boards {
    #[inline]
    fn eq(&self, other: &Boards) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Boards {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Boards::Rspi1A,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi1A");
                debug_trait_builder.finish()
            }
            (&Boards::Rspi1B,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi1B");
                debug_trait_builder.finish()
            }
            (&Boards::Rspi1APlus,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi1APlus");
                debug_trait_builder.finish()
            }
            (&Boards::Rspi1BPlus,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi1BPlus");
                debug_trait_builder.finish()
            }
            (&Boards::Rspi2B,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi2B");
                debug_trait_builder.finish()
            }
            (&Boards::RspiAlpha,) => {
                let mut debug_trait_builder = f.debug_tuple("RspiAlpha");
                debug_trait_builder.finish()
            }
            (&Boards::RspiCM1,) => {
                let mut debug_trait_builder = f.debug_tuple("RspiCM1");
                debug_trait_builder.finish()
            }
            (&Boards::Rspi3B,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi3B");
                debug_trait_builder.finish()
            }
            (&Boards::RspiZero,) => {
                let mut debug_trait_builder = f.debug_tuple("RspiZero");
                debug_trait_builder.finish()
            }
            (&Boards::RspiCM3,) => {
                let mut debug_trait_builder = f.debug_tuple("RspiCM3");
                debug_trait_builder.finish()
            }
            (&Boards::RspiZeroW,) => {
                let mut debug_trait_builder = f.debug_tuple("RspiZeroW");
                debug_trait_builder.finish()
            }
            (&Boards::Rspi3BPlus,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi3BPlus");
                debug_trait_builder.finish()
            }
            (&Boards::Rspi3APlus,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi3APlus");
                debug_trait_builder.finish()
            }
            (&Boards::RspiCM3Plus,) => {
                let mut debug_trait_builder = f.debug_tuple("RspiCM3Plus");
                debug_trait_builder.finish()
            }
            (&Boards::Rspi4B,) => {
                let mut debug_trait_builder = f.debug_tuple("Rspi4B");
                debug_trait_builder.finish()
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Boards: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Boards {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
                __field12,
                __field13,
                __field14,
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
                        2u64 => _serde::export::Ok(__Field::__field2),
                        3u64 => _serde::export::Ok(__Field::__field3),
                        4u64 => _serde::export::Ok(__Field::__field4),
                        5u64 => _serde::export::Ok(__Field::__field5),
                        6u64 => _serde::export::Ok(__Field::__field6),
                        7u64 => _serde::export::Ok(__Field::__field7),
                        8u64 => _serde::export::Ok(__Field::__field8),
                        9u64 => _serde::export::Ok(__Field::__field9),
                        10u64 => _serde::export::Ok(__Field::__field10),
                        11u64 => _serde::export::Ok(__Field::__field11),
                        12u64 => _serde::export::Ok(__Field::__field12),
                        13u64 => _serde::export::Ok(__Field::__field13),
                        14u64 => _serde::export::Ok(__Field::__field14),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 15",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "RSPi1A" => _serde::export::Ok(__Field::__field0),
                        "RSPi1B" => _serde::export::Ok(__Field::__field1),
                        "RSPi1APlus" => _serde::export::Ok(__Field::__field2),
                        "RSPi1BPlus" => _serde::export::Ok(__Field::__field3),
                        "RSPi2B" => _serde::export::Ok(__Field::__field4),
                        "RSPiAlpha" => _serde::export::Ok(__Field::__field5),
                        "RSPiCM1" => _serde::export::Ok(__Field::__field6),
                        "RSPi3B" => _serde::export::Ok(__Field::__field7),
                        "RSPiZero" => _serde::export::Ok(__Field::__field8),
                        "RSPiCM3" => _serde::export::Ok(__Field::__field9),
                        "RSPiZeroW" => _serde::export::Ok(__Field::__field10),
                        "RSPi3BPlus" => _serde::export::Ok(__Field::__field11),
                        "RSPi3APlus" => _serde::export::Ok(__Field::__field12),
                        "RSPiCM3Plus" => _serde::export::Ok(__Field::__field13),
                        "RSPi4B" => _serde::export::Ok(__Field::__field14),
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
                        b"RSPi1A" => _serde::export::Ok(__Field::__field0),
                        b"RSPi1B" => _serde::export::Ok(__Field::__field1),
                        b"RSPi1APlus" => _serde::export::Ok(__Field::__field2),
                        b"RSPi1BPlus" => _serde::export::Ok(__Field::__field3),
                        b"RSPi2B" => _serde::export::Ok(__Field::__field4),
                        b"RSPiAlpha" => _serde::export::Ok(__Field::__field5),
                        b"RSPiCM1" => _serde::export::Ok(__Field::__field6),
                        b"RSPi3B" => _serde::export::Ok(__Field::__field7),
                        b"RSPiZero" => _serde::export::Ok(__Field::__field8),
                        b"RSPiCM3" => _serde::export::Ok(__Field::__field9),
                        b"RSPiZeroW" => _serde::export::Ok(__Field::__field10),
                        b"RSPi3BPlus" => _serde::export::Ok(__Field::__field11),
                        b"RSPi3APlus" => _serde::export::Ok(__Field::__field12),
                        b"RSPiCM3Plus" => _serde::export::Ok(__Field::__field13),
                        b"RSPi4B" => _serde::export::Ok(__Field::__field14),
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
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<Boards>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Boards;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "enum Boards")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        (__Field::__field0, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi1A)
                        }
                        (__Field::__field1, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi1B)
                        }
                        (__Field::__field2, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi1APlus)
                        }
                        (__Field::__field3, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi1BPlus)
                        }
                        (__Field::__field4, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi2B)
                        }
                        (__Field::__field5, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::RspiAlpha)
                        }
                        (__Field::__field6, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::RspiCM1)
                        }
                        (__Field::__field7, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi3B)
                        }
                        (__Field::__field8, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::RspiZero)
                        }
                        (__Field::__field9, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::RspiCM3)
                        }
                        (__Field::__field10, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::RspiZeroW)
                        }
                        (__Field::__field11, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi3BPlus)
                        }
                        (__Field::__field12, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi3APlus)
                        }
                        (__Field::__field13, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::RspiCM3Plus)
                        }
                        (__Field::__field14, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(Boards::Rspi4B)
                        }
                    }
                }
            }
            const VARIANTS: &'static [&'static str] = &[
                "RSPi1A",
                "RSPi1B",
                "RSPi1APlus",
                "RSPi1BPlus",
                "RSPi2B",
                "RSPiAlpha",
                "RSPiCM1",
                "RSPi3B",
                "RSPiZero",
                "RSPiCM3",
                "RSPiZeroW",
                "RSPi3BPlus",
                "RSPi3APlus",
                "RSPiCM3Plus",
                "RSPi4B",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "boards",
                VARIANTS,
                __Visitor {
                    marker: _serde::export::PhantomData::<Boards>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Boards: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Boards {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                Boards::Rspi1A => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    0u32,
                    "RSPi1A",
                ),
                Boards::Rspi1B => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    1u32,
                    "RSPi1B",
                ),
                Boards::Rspi1APlus => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    2u32,
                    "RSPi1APlus",
                ),
                Boards::Rspi1BPlus => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    3u32,
                    "RSPi1BPlus",
                ),
                Boards::Rspi2B => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    4u32,
                    "RSPi2B",
                ),
                Boards::RspiAlpha => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    5u32,
                    "RSPiAlpha",
                ),
                Boards::RspiCM1 => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    6u32,
                    "RSPiCM1",
                ),
                Boards::Rspi3B => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    7u32,
                    "RSPi3B",
                ),
                Boards::RspiZero => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    8u32,
                    "RSPiZero",
                ),
                Boards::RspiCM3 => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    9u32,
                    "RSPiCM3",
                ),
                Boards::RspiZeroW => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    10u32,
                    "RSPiZeroW",
                ),
                Boards::Rspi3BPlus => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    11u32,
                    "RSPi3BPlus",
                ),
                Boards::Rspi3APlus => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    12u32,
                    "RSPi3APlus",
                ),
                Boards::RspiCM3Plus => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    13u32,
                    "RSPiCM3Plus",
                ),
                Boards::Rspi4B => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "boards",
                    14u32,
                    "RSPi4B",
                ),
            }
        }
    }
};
pub struct SchemaItemItemDevices {
    pub commentary: Option<String>,
    pub device: Vec<String>,
    pub required: Option<bool>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for SchemaItemItemDevices {
    #[inline]
    fn clone(&self) -> SchemaItemItemDevices {
        match *self {
            SchemaItemItemDevices {
                commentary: ref __self_0_0,
                device: ref __self_0_1,
                required: ref __self_0_2,
                type_: ref __self_0_3,
            } => SchemaItemItemDevices {
                commentary: ::core::clone::Clone::clone(&(*__self_0_0)),
                device: ::core::clone::Clone::clone(&(*__self_0_1)),
                required: ::core::clone::Clone::clone(&(*__self_0_2)),
                type_: ::core::clone::Clone::clone(&(*__self_0_3)),
            },
        }
    }
}
impl ::core::marker::StructuralPartialEq for SchemaItemItemDevices {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for SchemaItemItemDevices {
    #[inline]
    fn eq(&self, other: &SchemaItemItemDevices) -> bool {
        match *other {
            SchemaItemItemDevices {
                commentary: ref __self_1_0,
                device: ref __self_1_1,
                required: ref __self_1_2,
                type_: ref __self_1_3,
            } => match *self {
                SchemaItemItemDevices {
                    commentary: ref __self_0_0,
                    device: ref __self_0_1,
                    required: ref __self_0_2,
                    type_: ref __self_0_3,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &SchemaItemItemDevices) -> bool {
        match *other {
            SchemaItemItemDevices {
                commentary: ref __self_1_0,
                device: ref __self_1_1,
                required: ref __self_1_2,
                type_: ref __self_1_3,
            } => match *self {
                SchemaItemItemDevices {
                    commentary: ref __self_0_0,
                    device: ref __self_0_1,
                    required: ref __self_0_2,
                    type_: ref __self_0_3,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for SchemaItemItemDevices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            SchemaItemItemDevices {
                commentary: ref __self_0_0,
                device: ref __self_0_1,
                required: ref __self_0_2,
                type_: ref __self_0_3,
            } => {
                let mut debug_trait_builder = f.debug_struct("SchemaItemItemDevices");
                let _ = debug_trait_builder.field("commentary", &&(*__self_0_0));
                let _ = debug_trait_builder.field("device", &&(*__self_0_1));
                let _ = debug_trait_builder.field("required", &&(*__self_0_2));
                let _ = debug_trait_builder.field("type_", &&(*__self_0_3));
                debug_trait_builder.finish()
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_SchemaItemItemDevices: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SchemaItemItemDevices {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        2u64 => _serde::export::Ok(__Field::__field2),
                        3u64 => _serde::export::Ok(__Field::__field3),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 4",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "commentary" => _serde::export::Ok(__Field::__field0),
                        "device" => _serde::export::Ok(__Field::__field1),
                        "required" => _serde::export::Ok(__Field::__field2),
                        "type" => _serde::export::Ok(__Field::__field3),
                        _ => _serde::export::Ok(__Field::__ignore),
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
                        b"commentary" => _serde::export::Ok(__Field::__field0),
                        b"device" => _serde::export::Ok(__Field::__field1),
                        b"required" => _serde::export::Ok(__Field::__field2),
                        b"type" => _serde::export::Ok(__Field::__field3),
                        _ => _serde::export::Ok(__Field::__ignore),
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
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<SchemaItemItemDevices>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SchemaItemItemDevices;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(
                        __formatter,
                        "struct SchemaItemItemDevices",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<Option<String>>(
                        &mut __seq,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct SchemaItemItemDevices with 4 elements",
                            ));
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<Vec<String>>(
                        &mut __seq,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct SchemaItemItemDevices with 4 elements",
                            ));
                        }
                    };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<Option<bool>>(&mut __seq)
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct SchemaItemItemDevices with 4 elements",
                                ));
                            }
                        };
                    let __field3 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct SchemaItemItemDevices with 4 elements",
                                ));
                            }
                        };
                    _serde::export::Ok(SchemaItemItemDevices {
                        commentary: __field0,
                        device: __field1,
                        required: __field2,
                        type_: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<Option<String>> = _serde::export::None;
                    let mut __field1: _serde::export::Option<Vec<String>> = _serde::export::None;
                    let mut __field2: _serde::export::Option<Option<bool>> = _serde::export::None;
                    let mut __field3: _serde::export::Option<String> = _serde::export::None;
                    while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "commentary",
                                        ),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::export::Option::is_some(&__field1) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "device",
                                        ),
                                    );
                                }
                                __field1 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Vec<String>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::export::Option::is_some(&__field2) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "required",
                                        ),
                                    );
                                }
                                __field2 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<bool>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field3 => {
                                if _serde::export::Option::is_some(&__field3) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                __field3 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("commentary") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::export::Some(__field1) => __field1,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("device") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::export::Some(__field2) => __field2,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("required") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::export::Some(__field3) => __field3,
                        _serde::export::None => match _serde::private::de::missing_field("type") {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    _serde::export::Ok(SchemaItemItemDevices {
                        commentary: __field0,
                        device: __field1,
                        required: __field2,
                        type_: __field3,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["commentary", "device", "required", "type"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SchemaItemItemDevices",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<SchemaItemItemDevices>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_SchemaItemItemDevices: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SchemaItemItemDevices {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "SchemaItemItemDevices",
                false as usize + 1 + 1 + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "commentary",
                &self.commentary,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "device",
                &self.device,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "required",
                &self.required,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "type",
                &self.type_,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
pub struct SchemaItemItemDevicesItemItemNetworks {
    pub domain: String,
    pub port: i64,
    pub protocol: String,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for SchemaItemItemDevicesItemItemNetworks {
    #[inline]
    fn clone(&self) -> SchemaItemItemDevicesItemItemNetworks {
        match *self {
            SchemaItemItemDevicesItemItemNetworks {
                domain: ref __self_0_0,
                port: ref __self_0_1,
                protocol: ref __self_0_2,
            } => SchemaItemItemDevicesItemItemNetworks {
                domain: ::core::clone::Clone::clone(&(*__self_0_0)),
                port: ::core::clone::Clone::clone(&(*__self_0_1)),
                protocol: ::core::clone::Clone::clone(&(*__self_0_2)),
            },
        }
    }
}
impl ::core::marker::StructuralPartialEq for SchemaItemItemDevicesItemItemNetworks {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for SchemaItemItemDevicesItemItemNetworks {
    #[inline]
    fn eq(&self, other: &SchemaItemItemDevicesItemItemNetworks) -> bool {
        match *other {
            SchemaItemItemDevicesItemItemNetworks {
                domain: ref __self_1_0,
                port: ref __self_1_1,
                protocol: ref __self_1_2,
            } => match *self {
                SchemaItemItemDevicesItemItemNetworks {
                    domain: ref __self_0_0,
                    port: ref __self_0_1,
                    protocol: ref __self_0_2,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &SchemaItemItemDevicesItemItemNetworks) -> bool {
        match *other {
            SchemaItemItemDevicesItemItemNetworks {
                domain: ref __self_1_0,
                port: ref __self_1_1,
                protocol: ref __self_1_2,
            } => match *self {
                SchemaItemItemDevicesItemItemNetworks {
                    domain: ref __self_0_0,
                    port: ref __self_0_1,
                    protocol: ref __self_0_2,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for SchemaItemItemDevicesItemItemNetworks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            SchemaItemItemDevicesItemItemNetworks {
                domain: ref __self_0_0,
                port: ref __self_0_1,
                protocol: ref __self_0_2,
            } => {
                let mut debug_trait_builder =
                    f.debug_struct("SchemaItemItemDevicesItemItemNetworks");
                let _ = debug_trait_builder.field("domain", &&(*__self_0_0));
                let _ = debug_trait_builder.field("port", &&(*__self_0_1));
                let _ = debug_trait_builder.field("protocol", &&(*__self_0_2));
                debug_trait_builder.finish()
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_SchemaItemItemDevicesItemItemNetworks: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SchemaItemItemDevicesItemItemNetworks {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        2u64 => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 3",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "domain" => _serde::export::Ok(__Field::__field0),
                        "port" => _serde::export::Ok(__Field::__field1),
                        "protocol" => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Ok(__Field::__ignore),
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
                        b"domain" => _serde::export::Ok(__Field::__field0),
                        b"port" => _serde::export::Ok(__Field::__field1),
                        b"protocol" => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Ok(__Field::__ignore),
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
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<SchemaItemItemDevicesItemItemNetworks>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SchemaItemItemDevicesItemItemNetworks;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(
                        __formatter,
                        "struct SchemaItemItemDevicesItemItemNetworks",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SchemaItemItemDevicesItemItemNetworks with 3 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<i64>(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct SchemaItemItemDevicesItemItemNetworks with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct SchemaItemItemDevicesItemItemNetworks with 3 elements",
                                ));
                            }
                        };
                    _serde::export::Ok(SchemaItemItemDevicesItemItemNetworks {
                        domain: __field0,
                        port: __field1,
                        protocol: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<String> = _serde::export::None;
                    let mut __field1: _serde::export::Option<i64> = _serde::export::None;
                    let mut __field2: _serde::export::Option<String> = _serde::export::None;
                    while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "domain",
                                        ),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::export::Option::is_some(&__field1) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("port"),
                                    );
                                }
                                __field1 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::export::Option::is_some(&__field2) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "protocol",
                                        ),
                                    );
                                }
                                __field2 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("domain") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::export::Some(__field1) => __field1,
                        _serde::export::None => match _serde::private::de::missing_field("port") {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    let __field2 = match __field2 {
                        _serde::export::Some(__field2) => __field2,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("protocol") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::export::Ok(SchemaItemItemDevicesItemItemNetworks {
                        domain: __field0,
                        port: __field1,
                        protocol: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["domain", "port", "protocol"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SchemaItemItemDevicesItemItemNetworks",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<SchemaItemItemDevicesItemItemNetworks>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_SchemaItemItemDevicesItemItemNetworks: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SchemaItemItemDevicesItemItemNetworks {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "SchemaItemItemDevicesItemItemNetworks",
                false as usize + 1 + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "domain",
                &self.domain,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "port",
                &self.port,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "protocol",
                &self.protocol,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
pub struct Schema {
    pub boards: Vec<Boards>,
    pub devices: Vec<SchemaItemItemDevices>,
    pub networks: Option<Vec<SchemaItemItemDevicesItemItemNetworks>>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Schema {
    #[inline]
    fn clone(&self) -> Schema {
        match *self {
            Schema {
                boards: ref __self_0_0,
                devices: ref __self_0_1,
                networks: ref __self_0_2,
            } => Schema {
                boards: ::core::clone::Clone::clone(&(*__self_0_0)),
                devices: ::core::clone::Clone::clone(&(*__self_0_1)),
                networks: ::core::clone::Clone::clone(&(*__self_0_2)),
            },
        }
    }
}
impl ::core::marker::StructuralPartialEq for Schema {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Schema {
    #[inline]
    fn eq(&self, other: &Schema) -> bool {
        match *other {
            Schema {
                boards: ref __self_1_0,
                devices: ref __self_1_1,
                networks: ref __self_1_2,
            } => match *self {
                Schema {
                    boards: ref __self_0_0,
                    devices: ref __self_0_1,
                    networks: ref __self_0_2,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Schema) -> bool {
        match *other {
            Schema {
                boards: ref __self_1_0,
                devices: ref __self_1_1,
                networks: ref __self_1_2,
            } => match *self {
                Schema {
                    boards: ref __self_0_0,
                    devices: ref __self_0_1,
                    networks: ref __self_0_2,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Schema {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Schema {
                boards: ref __self_0_0,
                devices: ref __self_0_1,
                networks: ref __self_0_2,
            } => {
                let mut debug_trait_builder = f.debug_struct("Schema");
                let _ = debug_trait_builder.field("boards", &&(*__self_0_0));
                let _ = debug_trait_builder.field("devices", &&(*__self_0_1));
                let _ = debug_trait_builder.field("networks", &&(*__self_0_2));
                debug_trait_builder.finish()
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Schema: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Schema {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        2u64 => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 3",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "boards" => _serde::export::Ok(__Field::__field0),
                        "devices" => _serde::export::Ok(__Field::__field1),
                        "networks" => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Ok(__Field::__ignore),
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
                        b"boards" => _serde::export::Ok(__Field::__field0),
                        b"devices" => _serde::export::Ok(__Field::__field1),
                        b"networks" => _serde::export::Ok(__Field::__field2),
                        _ => _serde::export::Ok(__Field::__ignore),
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
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<Schema>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Schema;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "struct Schema")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<Vec<Boards>>(
                        &mut __seq,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct Schema with 3 elements",
                            ));
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                        Vec<SchemaItemItemDevices>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct Schema with 3 elements",
                            ));
                        }
                    };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                        Option<Vec<SchemaItemItemDevicesItemItemNetworks>>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct Schema with 3 elements",
                            ));
                        }
                    };
                    _serde::export::Ok(Schema {
                        boards: __field0,
                        devices: __field1,
                        networks: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<Vec<Boards>> = _serde::export::None;
                    let mut __field1: _serde::export::Option<Vec<SchemaItemItemDevices>> =
                        _serde::export::None;
                    let mut __field2: _serde::export::Option<
                        Option<Vec<SchemaItemItemDevicesItemItemNetworks>>,
                    > = _serde::export::None;
                    while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "boards",
                                        ),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Vec<Boards>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::export::Option::is_some(&__field1) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "devices",
                                        ),
                                    );
                                }
                                __field1 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        Vec<SchemaItemItemDevices>,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::export::Option::is_some(&__field2) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "networks",
                                        ),
                                    );
                                }
                                __field2 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<
                                        Option<Vec<SchemaItemItemDevicesItemItemNetworks>>,
                                    >(&mut __map)
                                    {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("boards") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::export::Some(__field1) => __field1,
                        _serde::export::None => match _serde::private::de::missing_field("devices")
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    let __field2 = match __field2 {
                        _serde::export::Some(__field2) => __field2,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("networks") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::export::Ok(Schema {
                        boards: __field0,
                        devices: __field1,
                        networks: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["boards", "devices", "networks"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Schema",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<Schema>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Schema: () = {
    #[allow(rust_2018_idioms, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Schema {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Schema",
                false as usize + 1 + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "boards",
                &self.boards,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "devices",
                &self.devices,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "networks",
                &self.networks,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nested: Schema = serde_json::from_str(r#"{ "append": "abc" }"#)?;
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&nested,) {
                (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
            },
        ));
    };
    Ok(())
}
