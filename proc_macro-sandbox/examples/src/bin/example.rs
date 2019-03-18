use foo_derive::*;
use foo::prelude::*;

#[derive(Debug, Foo)]
struct Bar{
    pub korosuzo: String
}

fn main() {
    Bar{korosuzo: "korosuzo".to_string()}.foo();
}
