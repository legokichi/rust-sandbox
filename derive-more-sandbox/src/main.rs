use std::convert::TryInto;
#[derive(derive_more::Display, Debug)]
struct Hoge(String);
fn main() {
    let hoge = Hoge("a".into());
    println!("{hoge}:{hoge:?}");
    let o = InbokeLambdaSuccessResponse::Foo(LambdaResponse::Ok(0));
    let o: LambdaResponse<i32, i32> = o.try_into().unwrap();
    println!("Hello, world!");
}
pub enum InbokeLambdaSuccessResponse {
    Foo(LambdaResponse<i32, i32>)
}
pub enum LambdaResponse<T, E> {
    Ok(T),
    Err(E),
}

impl ::core::convert::TryFrom<InbokeLambdaSuccessResponse> for LambdaResponse<i32, i32> {
    type Error = &'static str;
    fn try_from(value: InbokeLambdaSuccessResponse) -> Result<Self, Self::Error> {
        match value {
            InbokeLambdaSuccessResponse::Foo(o) => Ok(o),
            _ => Err(
                "Only Foo can be converted to LambdaResponse < i32, i32 >",
            ),
        }
    }
}
