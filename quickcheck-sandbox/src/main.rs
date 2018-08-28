#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[macro_use]
extern crate proptest;

fn main() {
    println!("Default config: {:?}", ::proptest::test_runner::Config::default());
    println!("{} + {} = {}", 1, 1, add2(1, 1));
}

use std::ops::Add;
fn add2<T: Add>(a: T, b: T) -> T::Output {
    a.add(b)
}




#[cfg(test)]
mod qc_tests {
    use add2;
    quickcheck! {
        fn prop(a: i32, b: i32) -> bool {
            println!("{} + {}", a, b);
            add2(a, b) == a + b
        }
    }
}


#[cfg(test)]
mod pt_tests {
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn doesnt_crash(s in "\\PC*") {
            println!("{}", s);
        }
    }
}

