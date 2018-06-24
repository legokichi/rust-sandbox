extern crate mio;
use std::io::Read;
use mio::*;
use mio::net::{TcpListener, TcpStream};
use std::collections::HashMap;

fn main() {
    let poll = Poll::new().unwrap();
    const SERVER: Token = Token(0);
    let mut clients: HashMap<Token, TcpStream> = HashMap::new();

    let addr = "127.0.0.1:13265".parse().unwrap();
    let server = TcpListener::bind(&addr).unwrap();
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge()).unwrap(); // edge trigger: 変化検出, level trigger: 状態通知
    let mut events: Events = Events::with_capacity(1024);
    let mut next_socket_index = 1usize;
    loop {
        poll.poll(&mut events, None).unwrap(); // timeout: None
        for event in &events {
            match event.token() {
                SERVER => {
                    loop {
                        match server.accept() {
                            Ok((stream, _addr)) => {
                                let token = Token(next_socket_index);
                                next_socket_index += 1;
                                poll.register(&stream, token, Ready::readable(), PollOpt::edge()).unwrap();
                                clients.insert(token, stream);
                            },
                            Err(ref e) if e.kind() == ::std::io::ErrorKind::WouldBlock => {
                                println!("suspend");
                                break;
                            },
                            e => panic!("err={:?}", e), // Unexpected error
                        }
                    }   
                }
                token => {
                    loop{
                        let mut buf = [0; 256];
                        match clients.get_mut(&token).unwrap().read(&mut buf) {
                            Ok(0) => {
                                println!("closed");
                                clients.remove(&token);
                                break;
                            },
                            Ok(size)=> {
                                let req = std::str::from_utf8(&buf).unwrap().to_string();
                                println!("{}, {:?}", size, req);
                            }
                            Err(ref e) if e.kind() == ::std::io::ErrorKind::WouldBlock => {
                                println!("suspend");
                                break;
                            }
                            e => panic!("err={:?}", e), // Unexpected error
                        }
                    }
                }
            }
        }
    }
    println!("Hello, world!");
}
