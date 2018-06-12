extern crate getopts;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
  pretty_env_logger::init();
  let a = 0;
  debug!("hello world {}", a);
}
/*
extern crate tokio_core;
extern crate mio;
extern crate futures;
extern crate futures_cpupool;
extern crate futures_fs;
extern crate bytes;
extern crate hyper;
extern crate hyper_tls;

use std::vec::{Vec};
use std::boxed::{Box};
use std::result::Result::{Ok, Err};
use std::path::{Path, PathBuf};
use std::time::{Duration};
use futures::{Future, Stream};
use futures::future;
use futures::stream;
use futures_cpupool::{CpuPool};
use futures_fs::FsPool;
use bytes::{Bytes};
use tokio_core::reactor::{Core, Timeout, Handle};
use hyper::{Body, Client, Get, Post, StatusCode, Chunk};
use hyper::server::{Http, Service, Request, Response};


struct WebService {
  handle: Handle, // for async network task
  fs: FsPool, // for async fileio stream task
  public_dir: PathBuf, // cwd
}

impl WebService {
  /// (PathBuf path) -> std::unique_ptr<std::fstream>
  fn static_file(&self, path: PathBuf) -> Result<Box<Stream<Item=Bytes, Error=std::io::Error>>, std::io::Error> {
    let path = if path == Path::new("/") { PathBuf::from("index.html") }else{ path };
    let path = if path.is_absolute() { PathBuf::from(path.to_str().unwrap()[1..].to_string()) }else{ path };
    // need cwd check here
    let filepath = self.public_dir.join(path);
    println!("{}", filepath.to_str().unwrap());
    match filepath.canonicalize() {
      Err(why) => Err(why),
      Ok(path) =>{
        let fin = self.fs.read(filepath);
        Ok(Box::new(fin)) // std::make_unique
      }
    }
  }
}

impl Service for WebService {
  type Request = Request;
  type Response = Response<Box<Stream<Item=Chunk, Error=Self::Error>>>;
  type Error = hyper::Error;
  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;
  fn call(&self, req: Self::Request) -> Self::Future {
    match (req.method(), req.path()) {
      (&Get, _) => {
        let body: Box<Stream<Item=Chunk, Error=Self::Error>> = match self.static_file(Path::new(req.path()).to_path_buf()){
          Ok(fin) => Box::new(fin.map(|byte| Chunk::from(byte)).map_err(|err| hyper::Error::Io(err))),
          Err(_) => Box::new(Body::from("Not Found".to_string())),
        };
        let res: Self::Response = Response::new().with_status(StatusCode::Ok).with_body(body);
        let fut = future::ok(res);
        Box::new(fut)
      },
      _ => {
        let body: Box<Stream<Item=Chunk, Error=Self::Error>> = Box::new(Body::from("Method Not Allowed".to_string()));
        let res: Self::Response = Response::new().with_status(StatusCode::MethodNotAllowed).with_body(body);
        let fut = future::ok(res);
        Box::new(fut)
      },
    }
  }
}

fn main() {
  pretty_env_logger::init();
  
  let args: Vec<String> = std::env::args().collect();
  let program = args[0].clone();
  let mut opts = getopts::Options::new();
  opts.optopt("p", "port", "port", "3000");
  opts.optflag("h", "help", "print this help menu");
  let matches = opts.parse(&args[1..]).unwrap();
  if matches.opt_present("h") {
    let help = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&help));
    return;
  }

  let port = matches.opt_str("p").unwrap();
  let addr = format!("127.0.0.1:{}", port).parse().unwrap();

  // async io resource
  let fspool = FsPool::new(4);
  let mut core = Core::new().unwrap(); // for network aio
  
  // create web service
  let server_handle = core.handle();
  let service_handle = core.handle();
  let serve = Http::new().serve_addr_handle(&addr, &server_handle, move ||{
    let service = WebService{
      handle: service_handle.clone(),
      fs: fspool.clone(),
      public_dir: std::env::current_dir().unwrap().as_path().to_path_buf(),
    };
    return Ok(service);
  }).unwrap();
  println!("Listening on http://{} with 1 thread.", serve.incoming_ref().local_addr());

  // launch web server
  let connection_handle = core.handle();
  server_handle.spawn(
    serve
      .for_each(move |conn| {
        // handle new connection
        connection_handle.spawn(
          conn
            .map(|_| ())
            .map_err(|err| println!("serve error: {:?}", err))
        );
        return Ok(());
      }).map_err(|_| ())
  );

  core.run(future::empty::<(), ()>()).unwrap();
}
*/