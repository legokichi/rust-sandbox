#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate failure;


use failure::{Backtrace, Context, Fail, ResultExt};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Fail, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ErrorKind {
    #[fail(display = "An Some error occurred. {}", message)]
    SomeError { message: String, some_args: Vec<i32>  },
}

fn main() {
    let txt = serde_json::to_string(&ErrorKind::SomeError{message: "a".into(), some_args: vec![1,2,3]}).unwrap();

    assert_eq!(txt, r##"{"type":"SomeError","message":"a","some_args":[1,2,3]}"##);

    let root: Result<(), failure::Error> = Err(format_err!("a"));
    let e: failure::Error = root.context(format_err!("b")).context(format_err!("c")).map_err(Into::into).unwrap_err();
    e.iter_chain().for_each(|e|{println!("{}", e);})
    
}
