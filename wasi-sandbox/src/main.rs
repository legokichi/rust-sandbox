// #![feature(link_args)]
// #[link_args = "-s USE_PTHREADS=1"]
// extern {}


fn main() {
    let th = std::thread::spawn(move || {
        println!("Hello, thread!");
    });
    println!("Hello, world!");
    th.join().unwrap();
}
