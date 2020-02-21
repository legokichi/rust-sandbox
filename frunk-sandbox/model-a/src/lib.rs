#[macro_use]
extern crate frunk;
#[macro_use]
extern crate derive_more;
extern crate frunk_enum_core;
#[macro_use]
extern crate frunk_enum_derive;

#[derive(Generic,LabelledGeneric, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref)]
pub struct NonEmptyString(pub String);
impl std::convert::TryFrom<String> for NonEmptyString {
    type Error = ();
    fn try_from(src: String) -> Result<Self, Self::Error>{
        Ok(Self(src))
    }
}

#[derive(Generic, LabelledGeneric, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Into, From, Deref)]
pub struct A(pub NonEmptyString);

#[derive(Generic,LabelledGeneric, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct A2{
    pub o: A,
}
#[derive(LabelledGeneric, Debug)]
pub struct A3 {
    pub name: String,
    pub age: u32,
}
#[derive(LabelledGeneric, Debug)]
pub struct A4 {
    pub name: String,
}

#[derive(LabelledGenericEnum)]
enum Foo{
    String(String)
}
