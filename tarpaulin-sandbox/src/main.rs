

#[cfg(not(tarpaulin_include))]
fn main() {
    let _a = a();
    let _b = b();
    println!("Hello, world!, {:?}", _a && _b);
}

fn a()->bool{
    true
}

#[test]
#[cfg(not(tarpaulin))]
fn a_test(){
    a();
    assert!(true);
}

fn b()->bool{
    true
}
#[test]
#[cfg(not(tarpaulin))]
fn b_test(){
    b();
    assert!(true);
}

