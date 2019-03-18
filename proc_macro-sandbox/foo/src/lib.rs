pub mod prelude {
    pub use crate::Foo;
}

pub trait Foo: ::std::fmt::Debug {
    fn foo(&self) -> () {
        println!("foo: {:?}", self);
    }
}
