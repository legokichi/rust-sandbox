use std::{future::Future, panic::UnwindSafe};
use std::pin::Pin;
fn main() {
    // Unpin は move させてもいい型
    // ほとんどの型は Unpin を実装している
    // が、例外は自己参照構造体を持つ generator こと async{} ブロック
    // および std::marker::PhantomPinned マーカー
    // `impl !Unpin for PhantomPinned` => Unpin できない => move できない
    
    struct A<T: Unpin>(T);

    // A(async{});
    // ^^^^^^^^^^ within `impl Future`, the trait `Unpin` is not implemented for `from_generator::GenFuture<[static generator@src/main.rs:9:12: 9:14]>`
    struct DoNotMove(std::marker::PhantomPinned);
    // A(DoNotMove(std::marker::PhantomPinned));
    // ^ within `DoNotMove`, the trait `Unpin` is not implemented for `PhantomPinned`

    // let a: Pin<Box<dyn Future<Output=()> + Send + Sync + 'static>> = Box::pin(async{
    //     let a = 0;
    //     A(&a);
    // });

    // async{} は空ブロックだから参照もなんも持ってないやんけと思うかもしれないけど、そういうものです

    // スタック上の参照はムーブさせてはいけないというだけで、
    // move 事態はできる
    A(Box::new(async{}));
    A(Box::new(DoNotMove(std::marker::PhantomPinned)));
    // drop(DoNotMove(std::marker::PhantomPinned));
    // drop(async {});

    // じゃあ Pin は何かって言うと スタック|ヒープ にピン留めされた型を作る型
    let mut a = DoNotMove(std::marker::PhantomPinned);
    let mut a = unsafe { Pin::new_unchecked(&mut a)}; // Pin 留めされた参照
    
    // std::mem::replace(dest, src)
    Some(std::marker::PhantomData::<()>)
        .map(sttc)
        .map(unpin)
        .map(send)
        .map(sync)
        .map(sized)
        .map(copy)
        .map(unwind_safe)
        .map(ref_unwind_safe)
        // .map(deref)
        // .map(deref_mut)
        // .map(as_ref)
        // .map(as_mut)
        .map(borrow)
        .map(borrow_mut)
        .map(clone)
        .map(to_owned)
        // .map(drop)
    ;

}
fn sttc<T>(o: T) -> T where T: 'static { o }
fn unpin<T>(o: T) -> T where T: Unpin { o }
fn send<T>(o: T) -> T where T: Send { o }
fn sync<T>(o: T) -> T where T: Sync { o }
fn sized<T>(o: T) -> T where T: Sized { o }
fn copy<T>(o: T) -> T where T: Copy { o }
fn unwind_safe<T>(o: T) -> T where T: std::panic::UnwindSafe { o }
fn ref_unwind_safe<T>(o: T) -> T where T: std::panic::RefUnwindSafe { o }
fn deref<T, S>(o: T) -> T where T: std::ops::Deref<Target=S> { o }
fn deref_mut<T, S>(o: T) -> T where T: std::ops::DerefMut<Target=S> { o }
fn as_ref<T, S>(o: T) -> T where T: AsRef<S> { o }
fn as_mut<T, S>(o: T) -> T where T: AsMut<S> { o }
fn borrow<T, S>(o: T) -> T where T: std::borrow::Borrow<S> { o }
fn borrow_mut<T, S>(o: T) -> T where T: std::borrow::BorrowMut<S> { o }
fn clone<T>(o: T) -> T where T: Clone { o }
fn to_owned<T>(o: T) -> T where T: ToOwned { o }
fn drop<T>(o: T) -> T where T: Drop { o }


// error[E0277]: `from_generator::GenFuture<[static generator@src/main.rs:7:19: 7:21]>` cannot be unpinned
//   --> src/main.rs:6:11
//    |
// 6  | fn e() -> impl Future<Output=()> + Send + Sync + 'static {
//    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ within `impl Future`, the trait `Unpin` is not implemented for `from_generator::GenFuture<[static generator@src/main.rs:7:19: 7:21]>`
//    |
//    = note: required because it appears within the type `impl Future`
//    = note: required because of the requirements on the impl of `Future` for `Box<impl Future>`
// fn e() -> impl Future<Output=()> + Send + Sync + 'static {
//     Box::new(async{})
// 
// fn e2() -> impl Future<Output=()> + Send + Sync + 'static {
//     Box::pin(async{})
// }
// fn f() -> Box<dyn Future<Output=()> + Send + Sync + 'static> {
//     Box::new(async{})
// }
// fn g() -> Pin<Box<dyn Future<Output=()> + Send + Sync + 'static>> {
//     Box::pin(async{})
// }
