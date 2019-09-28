use futures::prelude::*;

// #[cfg(test)]
// use mockiato::mockable;

#[mockall::automock(type P = ();type Q = ();type R = ();)]
trait Greeter {
    type P;
    type Q;
    type R;
    fn greet(&self, name: &str) -> Box<dyn Future<Item=String, Error=()>>;
    fn greet2<T>(&self, name: Self::P) -> Box<dyn Future<Item=Self::Q, Error=Self::R>>
    where
        T: 'static;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use mockall::*;
    #[cfg(test)]
    use mockall::predicate::*;

/*
    #[test]
    fn greet_the_world() {
        let mut greeter = GreeterMock::new();

        greeter
            .expect_greet(|arg| arg.partial_eq("world"))
            // .times(1..2)
            .returns_once(Box::new(futures::future::ok(String::from("Hello world"))));


        assert_eq!("Hello world", greeter.greet("world").wait().unwrap());
    }
    */
    #[test]
    fn greet_the_world2() {
        let mut mock = MockGreeter::new();
        mock.expect_greet()
            .with(eq("world"))
            .times(1)
            .returning(|x| Box::new(futures::future::ok(String::from("Hello world"))));
        assert_eq!("Hello world", mock.greet("world").wait().unwrap());
    }
}



