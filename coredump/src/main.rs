fn main() {
    let a = vec![1,2,3];
    std::thread::sleep(std::time::Duration::from_secs(120));
    println!("{a:?}");
}
