// https://brson.github.io/2016/11/30/starting-with-error-chain
// https://users.rust-lang.org/t/announcing-error-chain-a-library-for-consistent-and-reliable-rust-error-handling/6133
#[macro_use]
extern crate error_chain;
mod another_errors{
    error_chain!{
    }
}
mod errors{
    error_chain!{
        // pub する型の名前
        types {  MyError, MyErrorKind, MyResultExt, MyResult; }
        // 他の error-chain を使ったエラー型を MyErrorKind::Another として加える
        links {
            Another(crate::another_errors::Error, crate::another_errors::ErrorKind);
        }
        // 他のエラー型を MyErrorKind::Io として加える
        foreign_links {
            Io(::std::io::Error);
        }
        // ErrorKind::CannotOpenFile を加える
        errors {
            CannotOpenFile(path: String) {
                description("ファイルあけへんやんけ")
                display("ファイルあけられへんやんけ: '{}'", path)
            }
        }
    }
    // ここには
    // * struct MyError { .. }
    //     * Error トレイト
    //       * impl Error for MyError
    //       * impl Debug for MyError
    //       * impl Display for MyError
    //     * error-chain 拡張トレイト
    //       * impl error_chain::ChainedError for MyError
    //     * 変換系トレイト
    //       * impl From<MyErrorKind> for MyError
    //       * impl<'a> From<&'a str> for MyError
    //       * impl From<String> for MyError
    //       * impl From<another_errors::Error> for MyError
    // * enum MyErrorKind { Msg(String), Another(crate::another_errors::ErrorKind), Io(::std::io::Error), Parse, }
    //     * 表示系トレイト
    //       * impl Debug for MyErrorKind
    //       * impl Display for MyErrorKind
    //     * 変換系トレイト
    //       * impl<'a> From<&'a str> for MyErrorKind
    //       * impl From<String> for MyErrorKind
    //       * impl From<MyError> for MyErrorKind 
    //       * impl From<another_errors::ErrorKind> for MyErrorKind
    // * trait MyResultExt<T> { .. }
    // * type MyResult<T> = Result<T, MyError>;
}


fn main(){
    use errors::{MyError, MyErrorKind, MyResultExt};
    let ret: Result<_, std::io::Error> = std::fs::File::open("foo");
    let ret: Result<_, MyErrorKind> = ret.map_err(MyErrorKind::Io);
    let ret: Result<_, MyError> = ret.map_err(Into::into);
    let ret = ret.chain_err(|| "ファイル開けへんやんけ");
    let ret = ret.chain_err(|| MyErrorKind::CannotOpenFile("foo".to_string()));
    // let err = err.chain_err(|| MyErrorKind::Io(std::io::Error::new(std::io::ErrorKind::Other, "bal")));
    if let Err(err) = ret {
        for err in err.iter() {
            println!("{}", err);
        }
        // need RUST_BACKTRACE=1
        if let Some(trace) = err.backtrace() {
            // io::Error ではなく MyError が作られた時点のトレースが得られる
            println!("{:?}", trace);
        }
    }
}
