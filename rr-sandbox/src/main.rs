fn main() {
    let a = id(1);
    dbg!(a);
}

pub fn id(n: i32) -> i32{
    n
}
