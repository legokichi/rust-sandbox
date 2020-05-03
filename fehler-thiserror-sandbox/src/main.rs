use frunk::Coprod;
use frunk::coproduct::Coproduct;
use std::io;
use thiserror::Error;
use fehler::throws;

#[derive(Error, Debug)]
#[error("error1")]
pub struct Error1{}
#[derive(Error, Debug)]
#[error("error2")]
pub struct Error2{}
#[throws(Coprod!(io::Error, Error1, Error2))]
pub fn foo(){
    bar().map_err(Coproduct::embed)?;
}
#[throws(Coprod!(io::Error, Error1))]
pub fn bar(){
}

fn main() {
    // let () = foo()?;
    println!("Hello, world!");
}
