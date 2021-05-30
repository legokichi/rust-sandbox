

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
struct Struc {
    flag: bool,
    i_128: i128,
    i_64: i64,
    i_32: i32,
    i_16: i16,
    i_8: i8,
    u_128: u128,
    u_64: u64,
    u_32: u32,
    u_16: u16,
    u_8: u8,
    f_64: f64,
    f_32: f32,
    bytes: Vec<u8>,
    text: String,
    // enm: Enm,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum Enm {
    A,
    B,
    C(i128)
}


fn main() {
    let mut a = dbg!(serde_cbor::to_vec(&Struc{
        // text: "a".to_string(),
        // enm: Enm::C(128),
        // flag: true,
        // i_128: 0
        ..Default::default()
    }).unwrap());
    dbg!(a.len());
    let b = dbg!(serde_cbor::de::from_mut_slice::<serde_cbor::Value>(&mut a).unwrap());
    dbg!(serde_cbor::value::from_value::<Struc>(b).unwrap());
}
