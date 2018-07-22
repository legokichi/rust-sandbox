extern crate futures;
extern crate tokio;
extern crate tokio_uds;
extern crate tokio_timer;
#[macro_use]
extern crate mdo;
extern crate mdo_future;

use mdo_future::future::*;
use futures::prelude::*;
use futures::future;
use tokio_uds::UnixStream;

fn main() {
    let docker_sock = std::path::Path::new("/var/run/docker.sock");
    tokio::run(mdo!{
        mut client =<< UnixStream::connect(&docker_sock);
        _ =<< {
            use std::io::Write;
            let request = format!("GET /containers/json?all=1&size=1 HTTP/1.1\r\nHOST: rust\r\n\r\n");
            println!("{}", request);
            future::result(client.write_all(request.as_bytes()))
        };
        _ =<< tokio_timer::sleep(std::time::Duration::from_secs(1)).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", err)));
        _ =<< {
            use std::io::Read;
            const BUFFER_SIZE: usize = 1024;
            let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            let mut raw: Vec<u8> = Vec::new();
            loop {
                let ret = client.read(&mut buffer).map(|len|{
                    println!("{}", len);
                    for i in 0..len { raw.push(buffer[i]); }
                });
                if let Err(err) = ret {
                    if err.kind() == std::io::ErrorKind::WouldBlock {
                        break;
                    }
                    panic!("{:?}", err);
                }
            }
            let ret = std::str::from_utf8(&raw).unwrap();
            println!("{}", ret);
            future::ok(())
        };
        ret future::ok(())
    }.map_err(|_| () ));
}
