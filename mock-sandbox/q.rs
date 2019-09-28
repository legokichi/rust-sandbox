#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use futures::prelude::*;
#[cfg(test)]
use mockall::predicate::*;
#[cfg(test)]
use mockall::*;
trait Greeter {
    fn greet(&self, name: &str) -> Box<dyn Future<Item = String, Error = ()>>;
}
#[allow(non_snake_case)]
pub mod __mock_Greeter {
    use super::*;
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct MockGreeter {
    Greeter_expectations: MockGreeter_Greeter,
}
impl ::std::default::Default for MockGreeter {
    fn default() -> Self {
        Self {
            Greeter_expectations: MockGreeter_Greeter::default(),
        }
    }
}
#[allow(non_snake_case)]
pub mod __mock_Greeter_Greeter {
    use super::*;
    pub mod greet {
        use super::*;
        use ::mockall::CaseTreeExt;
        use ::std::{
            mem,
            ops::{DerefMut, Range},
            sync::Mutex,
        };
        enum Rfunc {
            Default,
            Expired,
            Mut(Box<dyn FnMut(&str) -> Box<dyn Future<Item = String, Error = ()>> + Send>),
            Once(Box<dyn FnOnce(&str) -> Box<dyn Future<Item = String, Error = ()>> + Send>),
            _Phantom(Box<dyn Fn() -> () + Send>),
        }
        impl Rfunc {
            fn call_mut(
                &mut self,
                name: &str,
            ) -> Result<Box<dyn Future<Item = String, Error = ()>>, &'static str> {
                match self {
                    Rfunc::Default => {
                        use ::mockall::ReturnDefault;
                        :: mockall :: DefaultReturner :: < Box < dyn Future < Item = String , Error = ( ) > > > :: return_default ( )
                    }
                    Rfunc::Expired => Err("called twice, but it returns by move"),
                    Rfunc::Mut(__mockall_f) => Ok(__mockall_f(name)),
                    Rfunc::Once(_) => {
                        if let Rfunc::Once(mut __mockall_f) = mem::replace(self, Rfunc::Expired) {
                            Ok(__mockall_f(name))
                        } else {
                            {
                                {
                                    ::std::rt::begin_panic(
                                        "internal error: entered unreachable code",
                                        &("tests/test.rs", 11u32, 18u32),
                                    )
                                }
                            }
                        }
                    }
                    Rfunc::_Phantom(_) => ::std::rt::begin_panic(
                        "internal error: entered unreachable code",
                        &("tests/test.rs", 11u32, 18u32),
                    ),
                }
            }
        }
        impl std::default::Default for Rfunc {
            fn default() -> Self {
                Rfunc::Default
            }
        }
        enum Matcher {
            Always,
            Func(Box<dyn Fn(&str) -> bool + Send>),
            Pred(Box<(Box<dyn ::mockall::Predicate<str> + Send>,)>),
            _Phantom(Box<dyn Fn() -> () + Send>),
        }
        impl Matcher {
            fn matches(&self, name: &str) -> bool {
                match self {
                    Matcher::Always => true,
                    Matcher::Func(__mockall_f) => __mockall_f(name),
                    Matcher::Pred(__mockall_pred) => [__mockall_pred.0.eval(name)]
                        .into_iter()
                        .all(|__mockall_x| *__mockall_x),
                    _ => ::std::rt::begin_panic(
                        "internal error: entered unreachable code",
                        &("tests/test.rs", 11u32, 18u32),
                    ),
                }
            }
        }
        impl Default for Matcher {
            #[allow(unused_variables)]
            fn default() -> Self {
                Matcher::Always
            }
        }
        impl ::std::fmt::Display for Matcher {
            fn fmt(&self, __mockall_fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    Matcher::Always => __mockall_fmt.write_fmt(::core::fmt::Arguments::new_v1(
                        &["<anything>"],
                        &match () {
                            () => [],
                        },
                    )),
                    Matcher::Func(_) => __mockall_fmt.write_fmt(::core::fmt::Arguments::new_v1(
                        &["<function>"],
                        &match () {
                            () => [],
                        },
                    )),
                    Matcher::Pred(__mockall_p) => {
                        __mockall_fmt.write_fmt(::core::fmt::Arguments::new_v1(
                            &[""],
                            &match (&__mockall_p.0,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                    _ => ::std::rt::begin_panic(
                        "internal error: entered unreachable code",
                        &("tests/test.rs", 11u32, 18u32),
                    ),
                }
            }
        }
        #[doc = r" Holds the stuff that is independent of the output type"]
        struct Common {
            matcher: Mutex<Matcher>,
            seq_handle: Option<::mockall::SeqHandle>,
            times: ::mockall::Times,
        }
        impl std::default::Default for Common {
            fn default() -> Self {
                Common {
                    matcher: Mutex::new(Matcher::default()),
                    seq_handle: None,
                    times: ::mockall::Times::new("MockGreeter::greet"),
                }
            }
        }
        impl Common {
            fn call(&self) {
                self.times.call().unwrap_or_else(|m| {
                    let desc = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &match (&self.matcher.lock().unwrap(),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    {
                        ::std::rt::begin_panic_fmt(
                            &::core::fmt::Arguments::new_v1(
                                &["", ": Expectation(", ") "],
                                &match (&"MockGreeter::greet", &desc, &m) {
                                    (arg0, arg1, arg2) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg2,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ),
                            &("tests/test.rs", 11u32, 18u32),
                        )
                    };
                });
                self.verify_sequence();
                if self.times.is_satisfied() {
                    self.satisfy_sequence()
                }
            }
            fn in_sequence(&mut self, __mockall_seq: &mut ::mockall::Sequence) -> &mut Self {
                if !self.times.is_exact() {
                    {
                        ::std::rt::begin_panic(
                            "Only Expectations with an exact call count have sequences",
                            &("tests/test.rs", 11u32, 18u32),
                        )
                    }
                };
                self.seq_handle = Some(__mockall_seq.next_handle());
                self
            }
            fn is_done(&self) -> bool {
                self.times.is_done()
            }
            fn matches(&self, name: &str) -> bool {
                self.matcher.lock().unwrap().matches(name)
            }
            #[doc = r" Forbid this expectation from ever being called."]
            fn never(&mut self) {
                self.times.never();
            }
            fn satisfy_sequence(&self) {
                if let Some(__mockall_handle) = &self.seq_handle {
                    __mockall_handle.satisfy()
                }
            }
            #[doc = r" Expect this expectation to be called any number of times"]
            #[doc = r" contained with the given range."]
            fn times<MockallR>(&mut self, __mockall_r: MockallR)
            where
                MockallR: Into<::mockall::TimesRange>,
            {
                self.times.times(__mockall_r)
            }
            fn with<MockallMatcher0: ::mockall::Predicate<str> + Send + 'static>(
                &mut self,
                name: MockallMatcher0,
            ) {
                let mut __mockall_guard = self.matcher.lock().unwrap();
                mem::replace(
                    __mockall_guard.deref_mut(),
                    Matcher::Pred(Box::new((Box::new(name),))),
                );
            }
            fn withf<MockallF>(&mut self, __mockall_f: MockallF)
            where
                MockallF: Fn(&str) -> bool + Send + 'static,
            {
                let mut __mockall_guard = self.matcher.lock().unwrap();
                mem::replace(
                    __mockall_guard.deref_mut(),
                    Matcher::Func(Box::new(__mockall_f)),
                );
            }
            fn verify_sequence(&self) {
                if let Some(__mockall_handle) = &self.seq_handle {
                    __mockall_handle.verify()
                }
            }
        }
        impl Drop for Common {
            fn drop(&mut self) {
                if !::std::thread::panicking() && !self.times.is_satisfied() {
                    let desc = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &match (&self.matcher.lock().unwrap(),) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    {
                        ::std::rt::begin_panic_fmt(
                            &::core::fmt::Arguments::new_v1(
                                &["", ": Expectation(", ") called fewer than ", " times"],
                                &match (&"MockGreeter::greet", &desc, &self.times.minimum()) {
                                    (arg0, arg1, arg2) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg2,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                },
                            ),
                            &("tests/test.rs", 11u32, 18u32),
                        )
                    };
                }
            }
        }
        #[doc = r" Expectation type for methods that return a `'static` type."]
        #[doc = r" This is the type returned by the `expect_*` methods."]
        pub(in super::super) struct Expectation {
            common: Common,
            rfunc: Mutex<Rfunc>,
        }
        impl Expectation {
            #[doc = r" Call this [`Expectation`] as if it were the real method."]
            #[doc(hidden)]
            pub(in super::super) fn call(
                &self,
                name: &str,
            ) -> Box<dyn Future<Item = String, Error = ()>> {
                self.common.call();
                self.rfunc
                    .lock()
                    .unwrap()
                    .call_mut(name)
                    .unwrap_or_else(|message| {
                        let desc = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[""],
                            &match (&self.common.matcher.lock().unwrap(),) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        {
                            ::std::rt::begin_panic_fmt(
                                &::core::fmt::Arguments::new_v1(
                                    &["", ": Expectation(", ") "],
                                    &match (&"MockGreeter::greet", &desc, &message) {
                                        (arg0, arg1, arg2) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg2,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    },
                                ),
                                &("tests/test.rs", 11u32, 18u32),
                            )
                        };
                    })
            }
            #[doc = r" Return a constant value from the `Expectation`"]
            #[doc = r""]
            #[doc = r" The output type must be `Clone`.  The compiler can't always"]
            #[doc = r" infer the proper type to use with this method; you will usually"]
            #[doc = r" need to specify it explicitly.  i.e. `return_const(42i32)`"]
            #[doc = r" instead of `return_const(42)`."]
            #[allow(unused_variables)]
            pub(in super::super) fn return_const<MockallOutput>(
                &mut self,
                __mockall_c: MockallOutput,
            ) -> &mut Self
            where
                MockallOutput:
                    Clone + Into<Box<dyn Future<Item = String, Error = ()>>> + Send + 'static,
            {
                self.returning(move |name| __mockall_c.clone().into())
            }
            #[doc = r" Supply an `FnOnce` closure that will provide the return value"]
            #[doc = r" for this Expectation.  This is useful for return types that"]
            #[doc = r" aren't `Clone`.  It will be an error to call this method"]
            #[doc = r" multiple times."]
            pub(in super::super) fn return_once<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF:
                    FnOnce(&str) -> Box<dyn Future<Item = String, Error = ()>> + Send + 'static,
            {
                {
                    let mut __mockall_guard = self.rfunc.lock().unwrap();
                    mem::replace(
                        __mockall_guard.deref_mut(),
                        Rfunc::Once(Box::new(__mockall_f)),
                    );
                }
                self
            }
            #[doc = r" Single-threaded version of [`return_once`](#method.return_once)."]
            #[doc = r" This is useful for return types that are neither `Send` nor"]
            #[doc = r" `Clone`."]
            #[doc = r""]
            #[doc = r" It is a runtime error to call the mock method from a different"]
            #[doc = r" thread than the one that originally called this method.  It is"]
            #[doc = r" also a runtime error to call the method more than once."]
            pub(in super::super) fn return_once_st<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF: FnOnce(&str) -> Box<dyn Future<Item = String, Error = ()>> + 'static,
            {
                let __mockall_fragile = ::mockall::Fragile::new(__mockall_f);
                let __mockall_fonce =
                    Box::new(move |name: &str| (__mockall_fragile.into_inner())(name));
                {
                    let mut __mockall_guard = self.rfunc.lock().unwrap();
                    mem::replace(__mockall_guard.deref_mut(), Rfunc::Once(__mockall_fonce));
                }
                self
            }
            #[doc = r" Supply a closure that will provide the return value for this"]
            #[doc = r" `Expectation`.  The method's arguments are passed to the closure"]
            #[doc = r" by value."]
            pub(in super::super) fn returning<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF:
                    FnMut(&str) -> Box<dyn Future<Item = String, Error = ()>> + Send + 'static,
            {
                {
                    let mut __mockall_guard = self.rfunc.lock().unwrap();
                    mem::replace(
                        __mockall_guard.deref_mut(),
                        Rfunc::Mut(Box::new(__mockall_f)),
                    );
                }
                self
            }
            #[doc = r" Single-threaded version of [`returning`](#method.returning)."]
            #[doc = r" Can be used when the argument or return type isn't `Send`."]
            #[doc = r""]
            #[doc = r" It is a runtime error to call the mock method from a different"]
            #[doc = r" thread than the one that originally called this method."]
            pub(in super::super) fn returning_st<MockallF>(
                &mut self,
                __mockall_f: MockallF,
            ) -> &mut Self
            where
                MockallF: FnMut(&str) -> Box<dyn Future<Item = String, Error = ()>> + 'static,
            {
                let mut __mockall_fragile = ::mockall::Fragile::new(__mockall_f);
                let __mockall_fmut = move |name: &str| (__mockall_fragile.get_mut())(name);
                {
                    let mut __mockall_guard = self.rfunc.lock().unwrap();
                    mem::replace(
                        __mockall_guard.deref_mut(),
                        Rfunc::Mut(Box::new(__mockall_fmut)),
                    );
                }
                self
            }
            #[doc = r" Add this expectation to a"]
            #[doc = r" [`Sequence`](../../../mockall/struct.Sequence.html)."]
            pub(in super::super) fn in_sequence(
                &mut self,
                __mockall_seq: &mut ::mockall::Sequence,
            ) -> &mut Self {
                self.common.in_sequence(__mockall_seq);
                self
            }
            fn is_done(&self) -> bool {
                self.common.is_done()
            }
            #[doc = r" Validate this expectation's matcher."]
            fn matches(&self, name: &str) -> bool {
                self.common.matches(name)
            }
            #[doc = r" Forbid this expectation from ever being called."]
            pub(in super::super) fn never(&mut self) -> &mut Self {
                self.common.never();
                self
            }
            #[doc = r" Create a new, default, [`Expectation`](struct.Expectation.html)"]
            pub(in super::super) fn new() -> Self {
                Self::default()
            }
            #[doc = r" Expect this expectation to be called exactly once.  Shortcut for"]
            #[doc = r" [`times(1)`](#method.times)."]
            pub(in super::super) fn once(&mut self) -> &mut Self {
                self.times(1)
            }
            #[doc = r" Restrict the number of times that that this method may be called."]
            #[doc = r""]
            #[doc = r" The argument may be:"]
            #[doc = r" * A fixed number: `.times(4)`"]
            #[doc = r" * Various types of range:"]
            #[doc = r"   - `.times(5..10)`"]
            #[doc = r"   - `.times(..10)`"]
            #[doc = r"   - `.times(5..)`"]
            #[doc = r"   - `.times(5..=10)`"]
            #[doc = r"   - `.times(..=10)`"]
            #[doc = r" * The wildcard: `.times(..)`"]
            pub(in super::super) fn times<MockallR>(&mut self, __mockall_r: MockallR) -> &mut Self
            where
                MockallR: Into<::mockall::TimesRange>,
            {
                self.common.times(__mockall_r);
                self
            }
            #[doc = r" Allow this expectation to be called any number of times"]
            #[doc = r""]
            #[doc = r" This behavior is the default, but the method is provided in case the"]
            #[doc = r" default behavior changes."]
            #[deprecated(since = "0.3.0", note = "Use times instead")]
            pub(in super::super) fn times_any(&mut self) -> &mut Self {
                self.common.times(..);
                self
            }
            #[doc = r" Allow this expectation to be called any number of times within a"]
            #[doc = r" given range"]
            #[deprecated(since = "0.3.0", note = "Use times instead")]
            pub(in super::super) fn times_range(
                &mut self,
                __mockall_range: Range<usize>,
            ) -> &mut Self {
                self.common.times(__mockall_range);
                self
            }
            #[doc = r" Set matching crieteria for this Expectation."]
            #[doc = r""]
            #[doc = r" The matching predicate can be anything implemening the"]
            #[doc = r" [`Predicate`](../../../mockall/trait.Predicate.html) trait.  Only"]
            #[doc = r" one matcher can be set per `Expectation` at a time."]
            pub(in super::super) fn with<
                MockallMatcher0: ::mockall::Predicate<str> + Send + 'static,
            >(
                &mut self,
                name: MockallMatcher0,
            ) -> &mut Self {
                self.common.with(name);
                self
            }
            #[doc = r" Set a matching function for this Expectation."]
            #[doc = r""]
            #[doc = r" This is equivalent to calling [`with`](#method.with) with a function"]
            #[doc = r" argument, like `with(predicate::function(f))`."]
            pub(in super::super) fn withf<MockallF>(&mut self, __mockall_f: MockallF) -> &mut Self
            where
                MockallF: Fn(&str) -> bool + Send + 'static,
            {
                self.common.withf(__mockall_f);
                self
            }
        }
        impl Default for Expectation {
            fn default() -> Self {
                Expectation {
                    common: Common::default(),
                    rfunc: Mutex::new(Rfunc::default()),
                }
            }
        }
        #[doc = r" A collection of [`Expectation`](struct.Expectations.html)"]
        #[doc = r" objects.  Users will rarely if ever use this struct directly."]
        #[doc(hidden)]
        pub(in super::super) struct Expectations(Vec<Expectation>);
        impl Expectations {
            #[doc = r" Verify that all current expectations are satisfied and clear"]
            #[doc = r" them."]
            pub(in super::super) fn checkpoint(&mut self) -> std::vec::Drain<Expectation> {
                self.0.drain(..)
            }
            #[doc = r" Create a new expectation for this method."]
            pub(in super::super) fn expect(&mut self) -> &mut Expectation {
                self.0.push(Expectation::default());
                let __mockall_l = self.0.len();
                &mut self.0[__mockall_l - 1]
            }
            pub(in super::super) fn new() -> Self {
                Self::default()
            }
        }
        impl Default for Expectations {
            fn default() -> Self {
                Expectations(Vec::new())
            }
        }
        impl Expectations {
            #[doc = r" Simulate calling the real method.  Every current expectation"]
            #[doc = r" will be checked in FIFO order and the first one with"]
            #[doc = r" matching arguments will be used."]
            pub(in super::super) fn call(
                &self,
                name: &str,
            ) -> Option<Box<dyn Future<Item = String, Error = ()>>> {
                self.0
                    .iter()
                    .find(|__mockall_e| {
                        __mockall_e.matches(name) && (!__mockall_e.is_done() || self.0.len() == 1)
                    })
                    .map(move |__mockall_e| __mockall_e.call(name))
            }
        }
        impl ::mockall::AnyExpectations for Expectations {}
    }
}
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct MockGreeter_Greeter {
    greet: __mock_Greeter_Greeter::greet::Expectations,
}
impl ::std::default::Default for MockGreeter_Greeter {
    fn default() -> Self {
        Self {
            greet: __mock_Greeter_Greeter::greet::Expectations::default(),
        }
    }
}
impl MockGreeter_Greeter {
    fn checkpoint(&mut self) {
        {
            self.greet.checkpoint();
        }
    }
}
impl MockGreeter {
    pub fn checkpoint(&mut self) {
        self.Greeter_expectations.checkpoint();
    }
    pub fn new() -> Self {
        Self::default()
    }
}
impl Greeter for MockGreeter {
    fn greet(&self, name: &str) -> Box<dyn Future<Item = String, Error = ()>> {
        self.Greeter_expectations
            .greet
            .call(name)
            .expect("No matching expectation found")
    }
}
impl MockGreeter {
    #[must_use = "Must set return value when not using the \"nightly\" feature"]
    fn expect_greet(&mut self) -> &mut __mock_Greeter_Greeter::greet::Expectation {
        self.Greeter_expectations.greet.expect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker]
    pub const greet_the_world2: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::greet_the_world2"),
            ignore: false,
            allow_fail: false,
            should_panic: test::ShouldPanic::No,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(greet_the_world2())),
    };
    fn greet_the_world2() {
        let mut mock = MockGreeter::new();
        mock.expect_greet()
            .with(eq("world"))
            .times(1)
            .returning(|x| Box::new(futures::future::ok(String::from("Hello world"))));
        {
            match (&"Hello world", &mock.greet("world").wait().unwrap()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            ::std::rt::begin_panic_fmt(
                                &::core::fmt::Arguments::new_v1(
                                    &[
                                        "assertion failed: `(left == right)`\n  left: `",
                                        "`,\n right: `",
                                        "`",
                                    ],
                                    &match (&&*left_val, &&*right_val) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Debug::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Debug::fmt,
                                            ),
                                        ],
                                    },
                                ),
                                &("tests/test.rs", 40u32, 9u32),
                            )
                        }
                    }
                }
            }
        };
    }
}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&greet_the_world2])
}
