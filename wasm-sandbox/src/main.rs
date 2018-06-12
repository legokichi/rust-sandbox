use std::thread;
use std::time;


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            let ten_millis = time::Duration::from_millis(10);
            std::thread::sleep(ten_millis);
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        let ten_millis = time::Duration::from_millis(10);
        std::thread::sleep(ten_millis);
    }
    
    handle.join();
}
