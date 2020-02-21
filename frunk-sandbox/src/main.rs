#[macro_use]
extern crate frunk;
use model_a::*;
use model_b::*;
use frunk::labelled::{Transmogrifier, LabelledGeneric};

fn a3tob3(a3: A3, friends: Vec<String>) -> B3 {
    let hlst = LabelledGeneric::into(a3);
    use frunk::labelled::chars::*;
    let hlst = hlst.prepend(field![(f,r,i,e,n,d,s), friends]);
    let tmp = hlst.transmogrify();
    let b3 = LabelledGeneric::from(tmp);
    b3
}

trait Inject {
    type Target;
    type Fields;
    fn inject(self, fields: Self::Fields) -> Self::Target;
}
use frunk::labelled::{Field, chars::*};
use frunk::{HCons, HNil};
impl Inject for A3{
    type Target = B3;
    type Fields = HCons<
        Field<(f, r, i, e, n, d, s), Vec<String>>,
        HNil
    >;
    fn inject(self, fields: Self::Fields) -> Self::Target {
        let hlst = LabelledGeneric::into(self);
        let (frends, _) = fields.pop();
        let hlst = hlst.prepend(frends);
        let tmp = hlst.transmogrify();
        let b3 = LabelledGeneric::from(tmp);
        b3
    }
}


fn main() {
    let a4 = A4{name: "a".into()};
    // let b4: B4 = a4.transmogrify();
}

