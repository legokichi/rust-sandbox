use clap::{App, Arg, SubCommand};
use dotenv::dotenv;
use env_logger::{Builder, Target};
use failure::{format_err, Error, Fail};
use structopt::StructOpt;
use log::*;
use serde::Deserialize;

#[derive(StructOpt)]
#[structopt(name = "battery-included", about = "buttery-included server and client", version="0")]
struct Opt {
    #[structopt(subcommand)]  
    cmd: Command
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "server")]
    Server {
        #[structopt(short = "p")]
        port: u16,
    },
    #[structopt(name = "client")]
    Client {
    },
}

fn main(){
    std::env::set_var("RUST_BACKTRACE", "full");
    std::env::set_var("RUST_LOG", "info");

    dotenv().ok();
    Builder::from_default_env().target(Target::Stderr).init();
    main2().unwrap();
    // if let Err(e) = main2() {
    //     e.iter_chain().enumerate().for_each(|(i, e)| {
    //         error!("{}: {}", i, e);
    //     });
    // }
}

fn main2() -> Result<(), Error> {
    let opt = Opt::from_iter_safe(std::env::args())?;
    match opt.cmd {
        Command::Client{} => { return client::init(); },
        Command::Server{port} => { return server::init(port); },
    }
    Ok(())
}
