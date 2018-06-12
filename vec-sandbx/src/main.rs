#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct V<T>(pub Vec<T>);
#[derive(Serialize, Deserialize, Debug)]
struct V_<T>(pub Vec<T>);
#[derive(Serialize, Deserialize, Debug)]
struct A(i32);
#[derive(Serialize, Deserialize, Debug)]
struct A_(i32);

impl From<V<A>> for V_<A_>{
  fn from(v:V<A>) -> Self {
    V_(v.0.into_iter().map(|A(a)| A_(a)).collect())
  }
}
impl<T> From<Vec<T>> for V_<T> {
  fn from(v: Vec<T>) -> V_<T> {
    V_(v)
  }
}
impl<T> From<Vec<T>> for V<T> {
  fn from(v: Vec<T>) -> V<T> {
    V(v)
  }
}
impl<T> Into<Vec<T>> for V<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}
impl<T> Into<Vec<T>> for V_<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}

fn main(){
  let a: V<A> = vec!(A(0)).into();
  println!("{:?}, {:?}", a, serde_json::to_string(&a));
  let a_: V_<A_> = a.into();
  println!("{:?}, {:?}", a_, serde_json::to_string(&a_));
}

