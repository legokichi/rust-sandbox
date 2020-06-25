use byteorder; // 1.3.4
use std::io::Write;
use byteorder::WriteBytesExt;
fn main(){
    let mut b = vec![];
    b.write_uint::<byteorder::BigEndian>(std::u64::MAX, 8).unwrap();
    println!("{:?}", b);
    println!("{:02X?}", b);

    println!("big");
    i64big::<byteorder::BigEndian>();
    println!("little");
    i64big::<byteorder::LittleEndian>();
}

fn i64big<T: byteorder::ByteOrder>(){
    let mut b = vec![];
    b.write_int::<T>(std::i64::MAX, 8).unwrap();
    println!("{}, {:?}", std::i64::MAX, b);
    let mut b = vec![];
    b.write_int::<T>(std::i64::MAX-1, 8).unwrap();
    println!("{}, {:?}", std::i64::MAX-1, b);
    let mut b = vec![];
    b.write_int::<T>(1, 8).unwrap();
    println!("{}, {:?}", 1, b);
    let mut b = vec![];
    b.write_int::<T>(0, 8).unwrap();
    println!("{} {:?}", 0, b);
    let mut b = vec![];
    b.write_int::<T>(-1, 8).unwrap();
    println!("{} {:?}", -1, b);
    let mut b = vec![];
    b.write_int::<T>(-2, 8).unwrap();
    println!("{} {:?}", -2, b);
    let mut b = vec![];
    b.write_int::<T>(std::i64::MIN+2, 8).unwrap();
    println!("{} {:?}", std::i64::MIN+2, b);
    let mut b = vec![];
    b.write_int::<T>(std::i64::MIN+1, 8).unwrap();
    println!("{} {:?}", std::i64::MIN+1, b);
    let mut b = vec![];
    b.write_int::<T>(std::i64::MIN, 8).unwrap();
    println!("{} {:?}", std::i64::MIN, b);
}
