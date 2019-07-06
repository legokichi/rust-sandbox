// https://blog.burntsushi.net/rust-error-handling/
// https://github.com/rust-lang/rfcs/blob/master/text/0201-error-chaining.md

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

// pub trait Error: Debug + Display { ... }
impl std::error::Error for MyError {
    fn description(&self) -> &str {
        match *self {
            MyError::Io(ref err) => err.description(),
            MyError::Parse(ref err) => err.description(),
        }
    }
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            MyError::Io(ref err) => Some(err),
            MyError::Parse(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MyError::Io(ref err) => write!(f, "ファイル開けへんやんけ: {}", err),
            MyError::Parse(ref err) => write!(f, "パースできへんやんけ: {}", err),
        }
    }
}

fn main(){
    use std::error::Error;
    let ret: Result<_, std::io::Error> = std::fs::File::open("foo");
    let ret Result<_, MyError> = ret.map_err(MyError::Io);
    if let Err(err) = ret {
        let mut cause = err.cause();
        while let Some(err) = cause {
            println!("{}", err);
            cause = err.cause();
        }
    }
}
