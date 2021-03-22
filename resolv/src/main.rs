use std::net::ToSocketAddrs;


fn main() {
    let th1 = std::thread::spawn(task("iot.actcast.io"));
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // let th2 = std::thread::spawn(task("c8ea15vhvqm3e.credentials.iot.ap-northeast-1.amazonaws.com"));
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // let th3 = std::thread::spawn(task("a1sgglpp228nnc-ats.iot.ap-northeast-1.amazonaws.com"));
    th1.join().unwrap();
    // th2.join().unwrap();
    // th3.join().unwrap();
}
fn task(host: &str)-> impl Fn()-> () {
    let host = host.to_owned();
    move ||{
        loop{
            match (host.as_ref(), 0).to_socket_addrs() {
                Ok(o) => {
                    // for p in o{
                    //     // dbg!(p);
                    // }
                },
                Err(err)=>{
                    // dbg!((err,&host));
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    }
}

// strace -tt -T -f ./target/release/resolv 2> log.txt