use err_derive::*;

#[derive(Debug, Error)]
#[error(display = "E1")]
struct E1(#[error(cause)] std::io::Error);

#[derive(Debug, Error)]
enum MyFuncError{
    #[error(display = "{}", _0)]
    E1(#[error(cause)] E1),
}

fn main()->Result<(), MyFuncError>{
    use std::error::Error;
    let a = MyFuncError::E1(E1(std::io::Error::new(std::io::ErrorKind::Other, "other".to_string())));
    let mut err = a.source();
    while let Some(ref e) = err {
        eprintln!("{}", e);
        err = e.source();
    }
    Ok(())
}
