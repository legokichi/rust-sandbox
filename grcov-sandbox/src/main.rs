fn main() {
    println!("Hello, world!");
}
fn hoge(){
    println!("hoge");
}
fn huga(){
    println!("huga");
}
#[test]
fn test(){
    hoge();
    huga();
    assert!(true);
}


