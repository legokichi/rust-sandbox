/// doc test - https://doc.rust-lang.org/beta/rustdoc/documentation-tests.html
/// 
/// # でその行をドキュメントに非表示
/// ```rust
/// let foo = "foo";
/// # assert_eq!(foo, "foo");
/// ```
/// 
/// ```rust
/// let foo = "foo";
/// assert_eq!(foo, "foo");
/// ```
/// 
/// ```ignore
/// テストされない
/// ```
/// 
/// ```should_panic
/// assert_eq!("かならず失敗するテスト".to_string().len(), 0);
/// ```
/// 
fn main() {
    for i in 1..10 {
        print_any(&A(i));
        print_any(&B(Box::new(i)));
        print_any2(&A(i));
        print_any3(&B(Box::new(i)));
    }
}

/// ジェネリクスの条件 = トレイト境界
#[derive(Debug)]
struct A<T: ::std::fmt::Debug>(T);

/// トレイトへの参照 = トレイトオブジェクト
#[derive(Debug)]
struct B(Box<::std::fmt::Debug>);

/// トレイトへの参照 = トレイトオブジェクト
fn print_any<'a>(object: &'a ::std::fmt::Debug) {
    // 関数はこれひとつ。動的ディスパッチなので実行時にデバッガからでも Debug 実装してる型なら呼べる
    println!("{:#?}", object);
}

/// ジェネリクスの条件 = トレイト境界
fn print_any2<'a, T: ::std::fmt::Debug>(object: &'a T){
    // コンパイル時にこの関数は T の型の数だけ作られる。
    println!("{:?}", object);
}

/// ジェネリクスの条件 = トレイト境界
fn print_any3<'a>(object: &'a impl ::std::fmt::Debug){
    // ジェネリクスの数だけ作られる
    println!("{:?}", object);
}

fn id(i: i32) -> i32 { return i; }