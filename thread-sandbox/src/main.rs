fn main() {
    println!("Hello, world!");
    let th = std::thread::Builder::new()
        .name("hoge".to_string())
        .spawn(move||{
            let th = std::thread::Builder::new()
                .spawn(move||{
                    loop{}
                }).unwrap();
            loop{}
            th.join().unwrap();
        }).unwrap();
    loop{}
    th.join().unwrap();
}
