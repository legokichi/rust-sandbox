extern crate nix;
use nix::sys::epoll::*;
use nix::sys::socket::*;
use nix::unistd::close;
use std::collections::HashMap;
use std::os::unix::io::RawFd;

/// TCP connection
/// Read -> Write -> End
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum State {
    Read,
    Write,
}

fn a()-> nix::Result<()> {
    let epfd = epoll_create()?;

    // epoll_event を入れる バッファ()
    // tokio では 1024 - https://github.com/tokio-rs/tokio/blob/c25ea78ec93f0eaa35bed3b61c7e98a408784a53/tokio-reactor/src/lib.rs#L248
    let mut epoll_events = vec![EpollEvent::empty(); 1024];
    let mut clients: HashMap<RawFd, State> = HashMap::new();
    
    // 接続待ち socket の fd
    let sockfd = socket(AddressFamily::Inet, SockType::Stream, SockFlag::SOCK_CLOEXEC, SockProtocol::Tcp)?;
    let addr = SockAddr::new_inet(InetAddr::new(IpAddr::new_v4(127, 0, 0, 1), 12345));
    println!("server fd: {}", sockfd);

    // local address に bind
    bind(sockfd, &addr)?;
    // backlog引数 が 1024 の理由 - https://github.com/carllerche/mio/pull/623
    listen(sockfd, 1024)?;

    // おそらく EpollEvent::new の引数の型が u64 なのは RawFd = i32 が -1 のときはエラーだから
    let mut ev = EpollEvent::new(EpollFlags::EPOLLIN, sockfd as u64);
    epoll_ctl(epfd, EpollOp::EpollCtlAdd, sockfd, &mut ev)?;
    
    loop{
        // 非同期 IO イベントが発生するまでスレッドをブロック
        //  -1 はタイムアウトなし ( 無限に待つ )
        let nfds = epoll_wait(epfd, &mut epoll_events, -1)?;
        // n 個の file descripter に何らかのイベントが発生した
        println!("epoll_wait: nfds={}", nfds);

        for i in 0..nfds {
            let data = epoll_events[i].data();
            let events = epoll_events[i].events();
            println!("i: {}, fd: {:?}, events: {:?}", i, data, events);
            let fd =  data as i32;
            // data にはイベントが発生した file descripter が入っている
            // events はそのイベントの状態がビットで格納されてる

            // 待受 socket が読み込み可能になった
            if fd == sockfd && events == events & EpollFlags::EPOLLIN {
                loop{
                    match accept(sockfd) {
                        Ok(client_fd) => {
                            println!("  accept client fd: {:?}", client_fd);
                            // client_fd を epoll の監視対象に入れて(EpollCtlAdd)
                            // read 可能になるのを待つ (client が http requeset を送信してくるのを待つ)
                            let mut ev = EpollEvent::new(EpollFlags::EPOLLIN | EpollFlags::EPOLLONESHOT, client_fd as u64);
                            epoll_ctl(epfd, EpollOp::EpollCtlAdd, client_fd, &mut ev)?;

                            clients.insert(client_fd, State::Read);
                            break;
                        },
                        Err(nix::Error::Sys(nix::errno::Errno::EAGAIN)) => {
                            println!("    suspended");
                            break;
                        },
                        err => { err?; },
                    }
                }
                continue;
            }
            // accept 済の client からの epoll event 
            if clients.contains_key(&fd) {
                let client_fd = fd;
                let state = *clients.get(&client_fd).unwrap();
                println!("  client_fd: {:?}, state: {:?}, events: {:?}", client_fd, state, events);

                if events == events & EpollFlags::EPOLLIN && state == State::Read {
                    loop{
                        let mut buf: [u8; 64] = [0; 64];
                        let size = recv(client_fd, &mut buf, MsgFlags::empty())?;
                        if size == 0 { println!("    closed"); break; }
                        let req = std::str::from_utf8(&buf[0..size]).unwrap().to_string();
                        println!("    recv: buf: {:?}", req);

                        // http request が終わるまで read し続ける
                        if !( req.find("\n\n").is_some() || req.find("\r\n\r\n").is_some() ){
                            continue;
                        }

                        // http request が終わった
                        // epoll の監視対象の client_fd を 
                        // write 可能になるのを待つように変更(EpollCtlMod)
                        let mut ev = EpollEvent::new(EpollFlags::EPOLLOUT, client_fd as u64);
                        epoll_ctl(epfd, EpollOp::EpollCtlMod, client_fd, &mut ev)?;

                        clients.insert(client_fd as i32, State::Write);
                        break;
                    }
                }else if events == events & EpollFlags::EPOLLOUT && state == State::Write {
                    // keep-aive 要求が来ても無視して Connection: close
                    let buf = "HTTP/1.1 200 Ok\nConnection: close\nContent-Type: text/plain\n\nha?\n\n";
                    let size = send(client_fd, buf.as_bytes(), MsgFlags::empty())?;
                    println!("    send: buf: {:?}, size: {}", buf, size);

                    // client_fd を epoll の監視対象から外す
                    epoll_ctl(epfd, EpollOp::EpollCtlDel, client_fd, &mut epoll_events[i])?;

                    // 接続中 client 一覧から削除
                    clients.remove(&client_fd);

                    // tcp 切断
                    shutdown(client_fd, Shutdown::Both)?;
                    close(client_fd)?;
                }
                continue;
            }
        }
    }
}

fn main() {
    println!("{:?}", a());
}
